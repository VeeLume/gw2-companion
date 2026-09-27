-- Migration 001: Initial schema
-- Sets up core tables for item cache, goals, and checklists.

-- Cached items from the API (avoids re-fetching known items)
CREATE TABLE IF NOT EXISTS items_cache (
    id          INTEGER PRIMARY KEY,
    name        TEXT NOT NULL,
    icon        TEXT,
    rarity      TEXT NOT NULL,
    item_type   TEXT NOT NULL,
    level       INTEGER NOT NULL DEFAULT 0,
    vendor_value INTEGER NOT NULL DEFAULT 0,
    data_json   TEXT NOT NULL,           -- full serialized Item JSON
    fetched_at  TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Cached recipes
CREATE TABLE IF NOT EXISTS recipes_cache (
    id              INTEGER PRIMARY KEY,
    output_item_id  INTEGER NOT NULL,
    output_count    INTEGER NOT NULL DEFAULT 1,
    min_rating      INTEGER NOT NULL DEFAULT 0,
    data_json       TEXT NOT NULL,
    fetched_at      TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Cached trading post prices (short TTL — refresh frequently)
CREATE TABLE IF NOT EXISTS price_cache (
    item_id     INTEGER PRIMARY KEY,
    buy_price   INTEGER NOT NULL DEFAULT 0,   -- copper
    sell_price  INTEGER NOT NULL DEFAULT 0,   -- copper
    buy_qty     INTEGER NOT NULL DEFAULT 0,
    sell_qty    INTEGER NOT NULL DEFAULT 0,
    fetched_at  TEXT NOT NULL DEFAULT (datetime('now'))
);

-- User's legendary crafting goals
CREATE TABLE IF NOT EXISTS legendary_goals (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    item_id     INTEGER NOT NULL,             -- the legendary item ID
    name        TEXT NOT NULL,
    created_at  TEXT NOT NULL DEFAULT (datetime('now')),
    completed   INTEGER NOT NULL DEFAULT 0    -- boolean
);

-- Tracked materials for a legendary goal
CREATE TABLE IF NOT EXISTS legendary_materials (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    goal_id     INTEGER NOT NULL REFERENCES legendary_goals(id) ON DELETE CASCADE,
    item_id     INTEGER NOT NULL,
    required    INTEGER NOT NULL DEFAULT 0,
    owned       INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_legendary_materials_goal
    ON legendary_materials(goal_id);

-- Daily / weekly checklist items
CREATE TABLE IF NOT EXISTS checklist_items (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    name        TEXT NOT NULL,
    category    TEXT NOT NULL,              -- 'daily', 'weekly', 'wizard_vault'
    reset_type  TEXT NOT NULL DEFAULT 'daily', -- 'daily', 'weekly', 'monthly'
    sort_order  INTEGER NOT NULL DEFAULT 0,
    active      INTEGER NOT NULL DEFAULT 1    -- boolean, user can disable
);

-- Checklist completion tracking
CREATE TABLE IF NOT EXISTS checklist_completions (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    checklist_id    INTEGER NOT NULL REFERENCES checklist_items(id) ON DELETE CASCADE,
    completed_at    TEXT NOT NULL DEFAULT (datetime('now')),
    reset_period    TEXT NOT NULL              -- e.g. '2025-03-01' for daily, '2025-W09' for weekly
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_checklist_completion_unique
    ON checklist_completions(checklist_id, reset_period);

-- User settings / API key storage
CREATE TABLE IF NOT EXISTS settings (
    key     TEXT PRIMARY KEY,
    value   TEXT NOT NULL
);
