/**
 * Type-safe wrappers around Tauri `invoke()` for calling Rust commands.
 */
import { invoke } from "@tauri-apps/api/core";

// === Types mirroring the Rust API types ===

export interface Item {
  id: number;
  chat_link: string;
  name: string;
  icon: string | null;
  description: string | null;
  item_type: string;
  rarity: string;
  level: number;
  vendor_value: number;
}

export interface ItemPrice {
  id: number;
  whitelisted: boolean;
  buys: { quantity: number; unit_price: number };
  sells: { quantity: number; unit_price: number };
}

export interface Account {
  id: string;
  name: string;
  world: number;
  created: string;
  access: string[];
  commander: boolean;
  fractal_level: number | null;
  daily_ap: number | null;
  monthly_ap: number | null;
  wvw: { team_id: number | null; rank: number | null } | null;
}

// === Command wrappers ===

export async function getAccountInfo(): Promise<Account> {
  return invoke("get_account_info");
}

export async function getItem(id: number): Promise<Item> {
  return invoke("get_item", { id });
}

export async function getItems(ids: number[]): Promise<Item[]> {
  return invoke("get_items", { ids });
}

export async function getItemPrice(id: number): Promise<ItemPrice> {
  return invoke("get_item_price", { id });
}

export async function searchRecipesByOutput(
  itemId: number
): Promise<number[]> {
  return invoke("search_recipes_by_output", { itemId });
}

export async function setApiKey(key: string): Promise<boolean> {
  return invoke("set_api_key", { key });
}

export async function getApiKeyStatus(): Promise<boolean> {
  return invoke("get_api_key_status");
}

// === Helpers ===

/** Format copper value as gold/silver/copper string. */
export function formatCoin(copper: number): string {
  const abs = Math.abs(copper);
  const g = Math.floor(abs / 10000);
  const s = Math.floor((abs % 10000) / 100);
  const c = abs % 100;
  const sign = copper < 0 ? "-" : "";

  if (g > 0) return `${sign}${g}g ${String(s).padStart(2, "0")}s ${String(c).padStart(2, "0")}c`;
  if (s > 0) return `${sign}${s}s ${String(c).padStart(2, "0")}c`;
  return `${sign}${c}c`;
}
