pub(crate) fn hover_info_detailed(name: &str) -> Option<String> {
    let lower = name.to_lowercase();
    match lower.as_str() {
        // Trading
        "ordersend" => Some(format!(
            "OrderSend — Send a trading order\n\
             \n\
             symbol:    Financial instrument name\n\
             cmd:       Trade operation type (OP_BUY/OP_SELL/etc)\n\
             volume:    Number of lots to trade\n\
             price:     Price at which to place the order\n\
             slippage:  Maximum price slippage in points\n\
             stoploss:  Stop loss level\n\
             takeprofit:Take profit level\n\
             \n\
             Returns: ticket number on success, -1 on failure"
        )),
        "orderselect" => Some(format!(
            "OrderSelect — Select an order for further processing\n\
             \n\
             index:  Order index in the selected pool\n\
             select: Selection mode (SELECT_BY_POS/SELECT_BY_TICKET)\n\
             pool:   Order pool (MODE_TRADES/MODE_HISTORY)\n\
             \n\
             Returns: true on success"
        )),
        "orderclose" => Some(format!(
            "OrderClose — Close an open order\n\
             \n\
             ticket:   Order ticket number\n\
             lots:     Number of lots to close\n\
             price:    Close price\n\
             slippage: Maximum price slippage in points\n\
             \n\
             Returns: true on success"
        )),
        "orderdelete" => Some(format!(
            "OrderDelete — Delete a pending order\n\
             \n\
             ticket: Order ticket number\n\
             \n\
             Returns: true on success"
        )),
        "ordermodify" => Some(format!(
            "OrderModify — Modify order parameters\n\
             \n\
             ticket:     Order ticket number\n\
             price:      New open price\n\
             stoploss:   New stop loss level\n\
             takeprofit: New take profit level\n\
             expiration: New order expiration time\n\
             arrow:      Arrow code for modification\n\
             \n\
             Returns: true on success"
        )),
        "orderstotal" => Some(format!(
            "OrdersTotal — Total number of open/closed orders\n\
             \n\
             Returns: number of orders in the selected pool"
        )),
        "positionselect" => Some(format!(
            "PositionSelect — Select a position by symbol\n\
             \n\
             symbol: Financial instrument name\n\
             \n\
             Returns: true if position exists"
        )),
        "imacd" => Some(format!(
            "iMACD — Moving Average Convergence Divergence indicator\n\
             \n\
             symbol:   Symbol name\n\
             timeframe:Timeframe\n\
             fast:     Fast EMA period\n\
             slow:     Slow EMA period\n\
             signal:   Signal line period\n\
             applied:  Applied price"
        )),
        "irsi" => Some(format!(
            "iRSI — Relative Strength Index indicator\n\
             \n\
             symbol:    Symbol name\n\
             timeframe: Timeframe\n\
             period:    Number of periods\n\
             applied:   Applied price"
        )),
        "ima" => Some(format!(
            "iMA — Moving Average indicator\n\
             \n\
             symbol:    Symbol name\n\
             timeframe: Timeframe\n\
             period:    Averaging period\n\
             shift:     Shift\n\
             method:    Averaging method\n\
             applied:   Applied price"
        )),
        "ibands" => Some(format!(
            "iBands — Bollinger Bands indicator\n\
             \n\
             symbol:    Symbol name\n\
             timeframe: Timeframe\n\
             period:    Period for main line\n\
             deviation: Standard deviations\n\
             shift:     Shift\n\
             applied:   Applied price"
        )),
        "istochastic" => Some(format!(
            "iStochastic — Stochastic Oscillator indicator\n\
             \n\
             symbol:    Symbol name\n\
             timeframe: Timeframe\n\
             K:         %K period\n\
             D:         %D period\n\
             slowing:   Slowing\n\
             method:    MA method\n\
             price:     Price field"
        )),
        "icci" => Some(format!(
            "iCCI — Commodity Channel Index indicator\n\
             \n\
             symbol:    Symbol name\n\
             timeframe: Timeframe\n\
             period:    Number of periods\n\
             applied:   Applied price"
        )),
        "iatr" => Some(format!(
            "iATR — Average True Range indicator\n\
             \n\
             symbol:    Symbol name\n\
             timeframe: Timeframe\n\
             period:    Averaging period"
        )),
        "ialligator" => Some(format!(
            "iAlligator — Alligator indicator\n\
             \n\
             symbol:    Symbol name\n\
             timeframe: Timeframe\n\
             jaw:       Jaw period\n\
             shift:     Jaw shift\n\
             teeth:     Teeth period\n\
             teeth_shift: Teeth shift\n\
             lips:      Lips period\n\
             lips_shift:Lips shift\n\
             method:    MA method\n\
             applied:   Applied price"
        )),
        "iadx" => Some(format!(
            "iADX — Average Directional Index indicator\n\
             \n\
             symbol:    Symbol name\n\
             timeframe: Timeframe\n\
             period:    Number of periods\n\
             applied:   Applied price"
        )),
        "imfi" => Some(format!(
            "iMFI — Money Flow Index indicator\n\
             \n\
             symbol:    Symbol name\n\
             timeframe: Timeframe\n\
             period:    Number of periods"
        )),
        "iobv" => Some(format!(
            "iOBV — On Balance Volume indicator\n\
             \n\
             symbol:    Symbol name\n\
             timeframe: Timeframe\n\
             applied:   Applied price"
        )),
        "ienvelopes" => Some(format!(
            "iEnvelopes — Envelopes indicator\n\
             \n\
             symbol:    Symbol name\n\
             timeframe: Timeframe\n\
             period:    Period for MA\n\
             method:    MA method\n\
             deviation: Deviation from MA\n\
             applied:   Applied price"
        )),
        "iforce" => Some(format!(
            "iForce — Force Index indicator\n\
             \n\
             symbol:    Symbol name\n\
             timeframe: Timeframe\n\
             period:    Number of periods\n\
             method:    MA method\n\
             applied:   Applied price"
        )),
        "iparabolic" | "isar" => Some(format!(
            "iSAR — Parabolic SAR indicator\n\
             \n\
             symbol:    Symbol name\n\
             timeframe: Timeframe\n\
             step:      Price increment step\n\
             maximum:   Maximum step"
        )),
        "normalizedouble" => Some(format!(
            "NormalizeDouble — Round a value to specified precision\n\
             \n\
             value:  Input floating-point number\n\
             digits: Number of decimal places\n\
             \n\
             Returns: the rounded value"
        )),
        "stringlen" => Some(format!(
            "StringLen — Get the length of a string\n\
             \n\
             str: Input string\n\
             \n\
             Returns: number of characters"
        )),
        "stringsubstr" => Some(format!(
            "StringSubstr — Extract a substring from a string\n\
             \n\
             str:    Source string\n\
             start:  Starting position (0-based)\n\
             length: Number of characters to extract\n\
             \n\
             Returns: extracted substring"
        )),
        "stringsplit" => Some(format!(
            "StringSplit — Split a string into substrings\n\
             \n\
             str:       String to split\n\
             separator: Delimiter character\n\
             result:    Array to receive the substrings\n\
             \n\
             Returns: number of substrings"
        )),
        "stringreplace" => Some(format!(
            "StringReplace — Replace occurrences of a substring\n\
             \n\
             str:    String to modify (passed by reference)\n\
             search: Substring to find\n\
             replace:Substring to replace with\n\
             \n\
             Returns: number of replacements"
        )),
        "fileopen" => Some(format!(
            "FileOpen — Open a file\n\
             \n\
             filename:  File path\n\
             flags:     Open mode flags (FILE_READ|FILE_WRITE|FILE_BIN|FILE_CSV|FILE_TXT)\n\
             delimiter: CSV delimiter (for FILE_CSV)\n\
             codepage:  Character encoding (CP_ACP or CP_UTF8)\n\
             \n\
             Returns: file handle, INVALID_HANDLE on failure"
        )),
        "timecurrent" => Some(format!(
            "TimeCurrent — Last known server time\n\
             \n\
             Returns: datetime value of the last known server time"
        )),
        "timelocal" => Some(format!(
            "TimeLocal — Local computer time\n\
             \n\
             Returns: datetime value of local computer time"
        )),
        "copyrates" => Some(format!(
            "CopyRates — Copy rate data into array\n\
             \n\
             symbol:     Symbol name\n\
             timeframe:  Timeframe\n\
             start:      Start position or start time\n\
             count:      Number of bars to copy\n\
             rates_array:Array of MqlRates to receive data\n\
             \n\
             Returns: number of copied bars, -1 on error"
        )),
        "chartid" => Some(format!(
            "ChartId — Get the ID of the current chart\n\
             \n\
             Returns: long chart identifier"
        )),
        "objectcreate" => Some(format!(
            "ObjectCreate — Create a chart object\n\
             \n\
             name:       Object name\n\
             type:       Object type (OBJ_* constant)\n\
             sub_window: Sub-window index (0 = main)\n\
             time1:      First anchor time\n\
             price1:     First anchor price\n\
             ...:        Additional anchor points depending on object type\n\
             \n\
             Returns: true on success"
        )),
        // Event handlers
        "ontick" => Some(format!(
            "OnTick — Expert Advisor tick event handler\n\
             \n\
             Called on every new tick. Main entry point for EAs.\n\
             \n\
             Parameters: none"
        )),
        "oninit" => Some(format!(
            "OnInit — Expert Advisor initialization handler\n\
             \n\
             Called when the EA is first loaded or symbol/timeframe changes.\n\
             \n\
             Parameters: none"
        )),
        "ondeinit" => Some(format!(
            "OnDeinit — Expert Advisor deinitialization handler\n\
             \n\
             reason: Reason for deinitialization (UNINITIALIZE_REASON_*)\n\
             \n\
             Called when the EA is being unloaded."
        )),
        "oncalculate" => Some(format!(
            "OnCalculate — Custom indicator calculation handler\n\
             \n\
             rates_total:     Total number of bars\n\
             prev_calculated: Number of bars calculated in previous call\n\
             begin:           Index of first meaningful bar\n\
             rates:           Array of rate data\n\
             \n\
             Returns: number of calculated bars"
        )),
        "onchartevent" => Some(format!(
            "OnChartEvent — Chart event handler\n\
             \n\
             id:     Event identifier (CHARTEVENT_*)\n\
             lparam: Long event parameter\n\
             dparam: Double event parameter\n\
             sparam: String event parameter"
        )),
        // Series
        "iclose" => Some(format!(
            "iClose — Close price of a bar\n\
             \n\
             symbol:    Symbol name\n\
             timeframe: Timeframe\n\
             shift:     Bar index (0 = current)\n\
             \n\
             Returns: close price value"
        )),
        "iopen" => Some(format!(
            "iOpen — Open price of a bar\n\
             \n\
             symbol:    Symbol name\n\
             timeframe: Timeframe\n\
             shift:     Bar index (0 = current)\n\
             \n\
             Returns: open price value"
        )),
        "ihigh" => Some(format!(
            "iHigh — High price of a bar\n\
             \n\
             symbol:    Symbol name\n\
             timeframe: Timeframe\n\
             shift:     Bar index (0 = current)\n\
             \n\
             Returns: high price value"
        )),
        "ilow" => Some(format!(
            "iLow — Low price of a bar\n\
             \n\
             symbol:    Symbol name\n\
             timeframe: Timeframe\n\
             shift:     Bar index (0 = current)\n\
             \n\
             Returns: low price value"
        )),

        _ => None,
    }
}
