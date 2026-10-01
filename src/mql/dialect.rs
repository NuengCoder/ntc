//! MQL4 vs MQL5 dialect split for built-in function names.
//!
//! The [`crate::mql::lexicon`] lists are dialect-unified. This module
//! curates the names that exist in only one dialect so completion can
//! filter per file extension (`.mq4` vs `.mq5`; `.mqh` shows the union).
//!
//! Best-effort v1: names present in both dialects — even with different
//! signatures (e.g. `iMA`, `SetIndexBuffer`) — are intentionally *shared*
//! rather than guessed wrong. Unknown names are shared too.

/// File dialect derived from the source extension.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Dialect {
    Mql4,
    Mql5,
}

/// Dialect for a file extension, or `None` for the union
/// (`.mqh` includes and anything unrecognized).
pub fn dialect_for_ext(ext: &str) -> Option<Dialect> {
    match ext.to_lowercase().as_str() {
        "mq4" => Some(Dialect::Mql4),
        "mq5" => Some(Dialect::Mql5),
        _ => None,
    }
}

/// Built-ins that exist only in MQL4 (classic order system, `Is*`/
/// `Account*` status functions, MQL4 indicator buffers, `Window*` API).
const MQL4_ONLY: &[&str] = &[
    // Classic order management
    "OrderSend",
    "OrderSelect",
    "OrderClose",
    "OrderDelete",
    "OrderModify",
    // MQL4 account status functions (MQL5 uses AccountInfo*)
    "AccountBalance",
    "AccountCredit",
    "AccountCompany",
    "AccountCurrency",
    "AccountEquity",
    "AccountFreeMargin",
    "AccountFreeMarginCheck",
    "AccountLeverage",
    "AccountMargin",
    "AccountName",
    "AccountNumber",
    "AccountProfit",
    "AccountServer",
    "AccountStopoutLevel",
    "AccountStopoutMode",
    // MQL4 terminal state (MQL5 uses TerminalInfo*/MQLInfo*)
    "IsConnected",
    "IsDemo",
    "IsDllsAllowed",
    "IsExpertEnabled",
    "IsLibrariesAllowed",
    "IsOptimization",
    "IsTesting",
    "IsTradeAllowed",
    "IsVisualMode",
    // MQL4 custom-indicator buffers and levels
    "IndicatorBuffers",
    "SetIndexStyle",
    "SetIndexArrow",
    "SetIndexDrawBegin",
    "SetIndexEmptyValue",
    "SetIndexLabel",
    "SetIndexShift",
    "SetLevelValue",
    "SetLevelStyle",
    "IndicatorDigits",
    "IndicatorShortName",
    // MQL4 chart windows (MQL5 uses Chart*)
    "WindowExpertName",
    "WindowHandle",
    "WindowFind",
    "WindowIsVisible",
    "WindowOnDropped",
    "WindowPriceMax",
    "WindowPriceMin",
    "WindowPriceOnDropped",
    "WindowBarsPerChart",
    "WindowScreenShot",
    "WindowXOnDropped",
    "WindowYOnDropped",
    "WindowTimeOnDropped",
    "WindowFirstVisibleBar",
    "WindowRedraw",
];

/// Built-ins that exist only in MQL5 (netting positions, history,
/// `Copy*` series, `AccountInfo*`, `SymbolInfo*`, resources, `Chart*`
/// management, MQL5-only events).
const MQL5_ONLY: &[&str] = &[
    // Netting positions
    "PositionSelect",
    "PositionSelectByTicket",
    "PositionGetTicket",
    "PositionGetInteger",
    "PositionGetDouble",
    "PositionGetString",
    "PositionsTotal",
    // History (deals and orders)
    "HistorySelect",
    "HistorySelectByPosition",
    "HistoryDealGetTicket",
    "HistoryDealGetInteger",
    "HistoryDealGetDouble",
    "HistoryDealGetString",
    "HistoryDealsTotal",
    "HistoryOrderGetTicket",
    "HistoryOrderGetInteger",
    "HistoryOrderGetDouble",
    "HistoryOrderGetString",
    "HistoryOrdersTotal",
    // Timeseries access
    "CopyRates",
    "CopyOpen",
    "CopyHigh",
    "CopyLow",
    "CopyClose",
    "CopyVolume",
    "CopyTime",
    "CopySpread",
    "CopyTickVolume",
    "CopyRealVolume",
    "Bars",
    "SeriesInfoInteger",
    // Account / terminal info
    "AccountInfoInteger",
    "AccountInfoDouble",
    "AccountInfoString",
    "TerminalInfoInteger",
    "TerminalInfoDouble",
    "TerminalInfoString",
    "UninitializeReason",
    // Symbols and market depth
    "SymbolInfoInteger",
    "SymbolInfoDouble",
    "SymbolInfoString",
    "SymbolSelect",
    "SymbolsTotal",
    "SymbolName",
    "MarketBookAdd",
    "MarketBookGet",
    "MarketBookRelease",
    // Time and ticks
    "TimeGMT",
    "TimeTradeServer",
    "GetTickCount64",
    // MQL5 indicator API
    "IndicatorSetInteger",
    "IndicatorSetDouble",
    "IndicatorSetString",
    "PlotIndexSetInteger",
    "PlotIndexSetDouble",
    "PlotIndexSetString",
    "IndicatorCreate",
    "IndicatorParameters",
    "IndicatorRelease",
    // Graphical resources
    "ResourceCreate",
    "ResourceReadImage",
    "ResourceSave",
    "TextSetFont",
    "TextOut",
    "TextGetSize",
    // Chart management
    "ChartOpen",
    "ChartClose",
    "ChartFirst",
    "ChartNext",
    "ChartNavigate",
    "ChartIndicatorAdd",
    "ChartIndicatorDelete",
    "ChartIndicatorGet",
    "ChartIndicatorsTotal",
    "ChartWindowFind",
    "ChartApplyTemplate",
    "ChartSaveTemplate",
    "ChartSetInteger",
    "ChartSetDouble",
    "ChartSetString",
    "ChartGetInteger",
    "ChartGetDouble",
    "ChartGetString",
    // MQL5-only events
    "OnTrade",
    "OnTradeTransaction",
    "OnBookEvent",
    "OnTester",
    "OnTesterPass",
    "OnTesterDeinit",
    "EventChartCustom",
];

/// True when `name` is visible in `dialect` (`None` = union: everything).
pub fn visible_in_dialect(name: &str, dialect: Option<Dialect>) -> bool {
    let Some(d) = dialect else {
        return true;
    };
    match d {
        Dialect::Mql4 => !MQL5_ONLY.iter().any(|n| n.eq_ignore_ascii_case(name)),
        Dialect::Mql5 => !MQL4_ONLY.iter().any(|n| n.eq_ignore_ascii_case(name)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mql::builtin_function_names;

    #[test]
    fn sets_are_disjoint() {
        for name in MQL4_ONLY {
            assert!(
                !MQL5_ONLY.iter().any(|m| m.eq_ignore_ascii_case(name)),
                "{} in both sets",
                name
            );
        }
    }

    #[test]
    fn sets_come_from_the_lexicon() {
        let all = builtin_function_names();
        for name in MQL4_ONLY.iter().chain(MQL5_ONLY.iter()) {
            assert!(
                all.iter().any(|n| n.eq_ignore_ascii_case(name)),
                "{} not in builtin_function_names",
                name
            );
        }
    }

    #[test]
    fn spot_checks() {
        assert!(!visible_in_dialect("OrderSend", Some(Dialect::Mql5)));
        assert!(visible_in_dialect("OrderSend", Some(Dialect::Mql4)));
        assert!(!visible_in_dialect("PositionSelect", Some(Dialect::Mql4)));
        assert!(visible_in_dialect("PositionSelect", Some(Dialect::Mql5)));
        // Shared names (and unknown ones) survive everywhere.
        for name in ["Print", "iMA", "OnTick", "SomethingMadeUp"] {
            assert!(visible_in_dialect(name, Some(Dialect::Mql4)));
            assert!(visible_in_dialect(name, Some(Dialect::Mql5)));
            assert!(visible_in_dialect(name, None));
        }
    }

    #[test]
    fn ext_mapping() {
        assert_eq!(dialect_for_ext("mq4"), Some(Dialect::Mql4));
        assert_eq!(dialect_for_ext("mq5"), Some(Dialect::Mql5));
        assert_eq!(dialect_for_ext("mqh"), None);
        assert_eq!(dialect_for_ext("MQ4"), Some(Dialect::Mql4));
        assert_eq!(dialect_for_ext("txt"), None);
    }
}
