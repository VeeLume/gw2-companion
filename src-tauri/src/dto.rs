//! The IPC view of gw2-api types.
//!
//! gw2-api's types carry no `specta::Type`, so commands never return them
//! directly: each has an app-owned view here, holding only the fields the UI
//! reads, and tauri-specta exports these to `src/lib/bindings.ts`. Adding a
//! field is one line here plus one in the `From` impl.
//!
//! Enums cross as their API strings, taken from their `Serialize` impl (which
//! round-trips unknown variants; `Display` exists only on lowercase enums), and
//! coin values as `f64` copper: specta rejects `i64`
//! (`BigIntForbidden`), and every realistic copper amount is exact in an f64.

use gw2_api::Coin;
use gw2_api::endpoints::{account::Account, commerce, items::Item};
use serde::Serialize;
use specta::Type;

fn copper(c: &Coin) -> f64 {
    c.0 as f64
}

/// The API string of a gw2-api enum.
fn api_str<T: Serialize>(v: &T) -> String {
    match serde_json::to_value(v) {
        Ok(serde_json::Value::String(s)) => s,
        _ => String::new(),
    }
}

/// `/v2/account`, as the dashboard shows it.
#[derive(Debug, Clone, Serialize, Type)]
pub struct AccountView {
    pub id: String,
    pub name: String,
    pub world: u32,
    pub created: String,
    /// Games and expansions the account owns, as API strings.
    pub access: Vec<String>,
    pub commander: bool,
    /// Needs the `progression` scope.
    pub fractal_level: Option<u32>,
    /// Needs the `progression` scope.
    pub daily_ap: Option<u32>,
    /// Needs the `progression` scope.
    pub monthly_ap: Option<u32>,
    /// Needs the `progression` scope.
    pub wvw_rank: Option<u32>,
    pub wvw_team_id: Option<u32>,
}

impl From<Account> for AccountView {
    fn from(a: Account) -> Self {
        Self {
            id: a.id,
            name: a.name,
            world: a.world,
            created: a.created,
            access: a.access.iter().map(api_str).collect(),
            commander: a.commander,
            fractal_level: a.fractal_level,
            daily_ap: a.daily_ap,
            monthly_ap: a.monthly_ap,
            wvw_rank: a.wvw.as_ref().and_then(|w| w.rank),
            wvw_team_id: a.wvw.as_ref().and_then(|w| w.team_id),
        }
    }
}

/// An item's common fields; the type-specific `details` are not exposed yet.
#[derive(Debug, Clone, Serialize, Type)]
pub struct ItemView {
    pub id: u32,
    pub chat_link: String,
    pub name: String,
    pub icon: Option<String>,
    pub description: Option<String>,
    pub item_type: String,
    pub rarity: String,
    pub level: u32,
    /// Copper.
    pub vendor_value: f64,
}

impl From<Item> for ItemView {
    fn from(i: Item) -> Self {
        Self {
            id: i.id.0,
            chat_link: i.chat_link,
            name: i.name,
            icon: i.icon,
            description: i.description,
            item_type: api_str(&i.item_type),
            rarity: api_str(&i.rarity),
            level: i.level,
            vendor_value: copper(&i.vendor_value),
        }
    }
}

/// One side of the trading post for an item.
#[derive(Debug, Clone, Serialize, Type)]
pub struct PriceSide {
    pub quantity: u32,
    /// Copper.
    pub unit_price: f64,
}

impl From<commerce::PriceInfo> for PriceSide {
    fn from(p: commerce::PriceInfo) -> Self {
        Self {
            quantity: p.quantity,
            unit_price: copper(&p.unit_price),
        }
    }
}

/// `/v2/commerce/prices` for one item.
#[derive(Debug, Clone, Serialize, Type)]
pub struct ItemPriceView {
    pub id: u32,
    pub whitelisted: bool,
    pub buys: PriceSide,
    pub sells: PriceSide,
}

impl From<commerce::ItemPrice> for ItemPriceView {
    fn from(p: commerce::ItemPrice) -> Self {
        Self {
            id: p.id.0,
            whitelisted: p.whitelisted,
            buys: p.buys.into(),
            sells: p.sells.into(),
        }
    }
}
