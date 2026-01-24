//! ISO 4217 Currency definitions, published on 2026-01-01
//!
//! source: <https://www.six-group.com/dam/download/financial-information/data-center/iso-currrency/lists/list-one.xml>

use crate::Currency;

/// Afghani
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct AFN;
impl Currency for AFN {
    fn code(&self) -> &'static str {
        "AFN"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        971
    }
}

/// Euro
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct EUR;
impl Currency for EUR {
    fn code(&self) -> &'static str {
        "EUR"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        978
    }
}

/// Lek
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ALL;
impl Currency for ALL {
    fn code(&self) -> &'static str {
        "ALL"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        8
    }
}

/// Algerian Dinar
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct DZD;
impl Currency for DZD {
    fn code(&self) -> &'static str {
        "DZD"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        12
    }
}

/// US Dollar
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct USD;
impl Currency for USD {
    fn code(&self) -> &'static str {
        "USD"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        840
    }
}

/// Kwanza
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct AOA;
impl Currency for AOA {
    fn code(&self) -> &'static str {
        "AOA"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        973
    }
}

/// East Caribbean Dollar
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct XCD;
impl Currency for XCD {
    fn code(&self) -> &'static str {
        "XCD"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        951
    }
}

/// Arab Accounting Dinar
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct XAD;
impl Currency for XAD {
    fn code(&self) -> &'static str {
        "XAD"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        396
    }
}

/// Argentine Peso
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ARS;
impl Currency for ARS {
    fn code(&self) -> &'static str {
        "ARS"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        32
    }
}

/// Armenian Dram
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct AMD;
impl Currency for AMD {
    fn code(&self) -> &'static str {
        "AMD"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        51
    }
}

/// Aruban Florin
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct AWG;
impl Currency for AWG {
    fn code(&self) -> &'static str {
        "AWG"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        533
    }
}

/// Australian Dollar
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct AUD;
impl Currency for AUD {
    fn code(&self) -> &'static str {
        "AUD"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        36
    }
}

/// Azerbaijan Manat
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct AZN;
impl Currency for AZN {
    fn code(&self) -> &'static str {
        "AZN"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        944
    }
}

/// Bahamian Dollar
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct BSD;
impl Currency for BSD {
    fn code(&self) -> &'static str {
        "BSD"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        44
    }
}

/// Bahraini Dinar
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct BHD;
impl Currency for BHD {
    fn code(&self) -> &'static str {
        "BHD"
    }

    fn minor_units(&self) -> u32 {
        3
    }

    fn numeric_code(&self) -> u32 {
        48
    }
}

/// Taka
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct BDT;
impl Currency for BDT {
    fn code(&self) -> &'static str {
        "BDT"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        50
    }
}

/// Barbados Dollar
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct BBD;
impl Currency for BBD {
    fn code(&self) -> &'static str {
        "BBD"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        52
    }
}

/// Belarusian Ruble
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct BYN;
impl Currency for BYN {
    fn code(&self) -> &'static str {
        "BYN"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        933
    }
}

/// Belize Dollar
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct BZD;
impl Currency for BZD {
    fn code(&self) -> &'static str {
        "BZD"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        84
    }
}

/// CFA Franc BCEAO
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct XOF;
impl Currency for XOF {
    fn code(&self) -> &'static str {
        "XOF"
    }

    fn minor_units(&self) -> u32 {
        0
    }

    fn numeric_code(&self) -> u32 {
        952
    }
}

/// Bermudian Dollar
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct BMD;
impl Currency for BMD {
    fn code(&self) -> &'static str {
        "BMD"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        60
    }
}

/// Indian Rupee
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct INR;
impl Currency for INR {
    fn code(&self) -> &'static str {
        "INR"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        356
    }
}

/// Ngultrum
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct BTN;
impl Currency for BTN {
    fn code(&self) -> &'static str {
        "BTN"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        64
    }
}

/// Boliviano
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct BOB;
impl Currency for BOB {
    fn code(&self) -> &'static str {
        "BOB"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        68
    }
}

/// Mvdol
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct BOV;
impl Currency for BOV {
    fn code(&self) -> &'static str {
        "BOV"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        984
    }
}

/// Convertible Mark
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct BAM;
impl Currency for BAM {
    fn code(&self) -> &'static str {
        "BAM"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        977
    }
}

/// Pula
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct BWP;
impl Currency for BWP {
    fn code(&self) -> &'static str {
        "BWP"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        72
    }
}

/// Norwegian Krone
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct NOK;
impl Currency for NOK {
    fn code(&self) -> &'static str {
        "NOK"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        578
    }
}

/// Brazilian Real
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct BRL;
impl Currency for BRL {
    fn code(&self) -> &'static str {
        "BRL"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        986
    }
}

/// Brunei Dollar
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct BND;
impl Currency for BND {
    fn code(&self) -> &'static str {
        "BND"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        96
    }
}

/// Burundi Franc
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct BIF;
impl Currency for BIF {
    fn code(&self) -> &'static str {
        "BIF"
    }

    fn minor_units(&self) -> u32 {
        0
    }

    fn numeric_code(&self) -> u32 {
        108
    }
}

/// Cabo Verde Escudo
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct CVE;
impl Currency for CVE {
    fn code(&self) -> &'static str {
        "CVE"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        132
    }
}

/// Riel
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct KHR;
impl Currency for KHR {
    fn code(&self) -> &'static str {
        "KHR"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        116
    }
}

/// CFA Franc BEAC
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct XAF;
impl Currency for XAF {
    fn code(&self) -> &'static str {
        "XAF"
    }

    fn minor_units(&self) -> u32 {
        0
    }

    fn numeric_code(&self) -> u32 {
        950
    }
}

/// Canadian Dollar
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct CAD;
impl Currency for CAD {
    fn code(&self) -> &'static str {
        "CAD"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        124
    }
}

/// Cayman Islands Dollar
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct KYD;
impl Currency for KYD {
    fn code(&self) -> &'static str {
        "KYD"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        136
    }
}

/// Chilean Peso
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct CLP;
impl Currency for CLP {
    fn code(&self) -> &'static str {
        "CLP"
    }

    fn minor_units(&self) -> u32 {
        0
    }

    fn numeric_code(&self) -> u32 {
        152
    }
}

/// Unidad de Fomento
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct CLF;
impl Currency for CLF {
    fn code(&self) -> &'static str {
        "CLF"
    }

    fn minor_units(&self) -> u32 {
        4
    }

    fn numeric_code(&self) -> u32 {
        990
    }
}

/// Yuan Renminbi
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct CNY;
impl Currency for CNY {
    fn code(&self) -> &'static str {
        "CNY"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        156
    }
}

/// Colombian Peso
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct COP;
impl Currency for COP {
    fn code(&self) -> &'static str {
        "COP"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        170
    }
}

/// Unidad de Valor Real
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct COU;
impl Currency for COU {
    fn code(&self) -> &'static str {
        "COU"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        970
    }
}

/// Comorian Franc
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct KMF;
impl Currency for KMF {
    fn code(&self) -> &'static str {
        "KMF"
    }

    fn minor_units(&self) -> u32 {
        0
    }

    fn numeric_code(&self) -> u32 {
        174
    }
}

/// Congolese Franc
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct CDF;
impl Currency for CDF {
    fn code(&self) -> &'static str {
        "CDF"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        976
    }
}

/// New Zealand Dollar
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct NZD;
impl Currency for NZD {
    fn code(&self) -> &'static str {
        "NZD"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        554
    }
}

/// Costa Rican Colon
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct CRC;
impl Currency for CRC {
    fn code(&self) -> &'static str {
        "CRC"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        188
    }
}

/// Cuban Peso
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct CUP;
impl Currency for CUP {
    fn code(&self) -> &'static str {
        "CUP"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        192
    }
}

/// Caribbean Guilder
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct XCG;
impl Currency for XCG {
    fn code(&self) -> &'static str {
        "XCG"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        532
    }
}

/// Czech Koruna
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct CZK;
impl Currency for CZK {
    fn code(&self) -> &'static str {
        "CZK"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        203
    }
}

/// Danish Krone
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct DKK;
impl Currency for DKK {
    fn code(&self) -> &'static str {
        "DKK"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        208
    }
}

/// Djibouti Franc
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct DJF;
impl Currency for DJF {
    fn code(&self) -> &'static str {
        "DJF"
    }

    fn minor_units(&self) -> u32 {
        0
    }

    fn numeric_code(&self) -> u32 {
        262
    }
}

/// Dominican Peso
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct DOP;
impl Currency for DOP {
    fn code(&self) -> &'static str {
        "DOP"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        214
    }
}

/// Egyptian Pound
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct EGP;
impl Currency for EGP {
    fn code(&self) -> &'static str {
        "EGP"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        818
    }
}

/// El Salvador Colon
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct SVC;
impl Currency for SVC {
    fn code(&self) -> &'static str {
        "SVC"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        222
    }
}

/// Nakfa
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ERN;
impl Currency for ERN {
    fn code(&self) -> &'static str {
        "ERN"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        232
    }
}

/// Lilangeni
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct SZL;
impl Currency for SZL {
    fn code(&self) -> &'static str {
        "SZL"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        748
    }
}

/// Ethiopian Birr
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ETB;
impl Currency for ETB {
    fn code(&self) -> &'static str {
        "ETB"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        230
    }
}

/// Falkland Islands Pound
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct FKP;
impl Currency for FKP {
    fn code(&self) -> &'static str {
        "FKP"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        238
    }
}

/// Fiji Dollar
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct FJD;
impl Currency for FJD {
    fn code(&self) -> &'static str {
        "FJD"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        242
    }
}

/// CFP Franc
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct XPF;
impl Currency for XPF {
    fn code(&self) -> &'static str {
        "XPF"
    }

    fn minor_units(&self) -> u32 {
        0
    }

    fn numeric_code(&self) -> u32 {
        953
    }
}

/// Dalasi
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct GMD;
impl Currency for GMD {
    fn code(&self) -> &'static str {
        "GMD"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        270
    }
}

/// Lari
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct GEL;
impl Currency for GEL {
    fn code(&self) -> &'static str {
        "GEL"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        981
    }
}

/// Ghana Cedi
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct GHS;
impl Currency for GHS {
    fn code(&self) -> &'static str {
        "GHS"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        936
    }
}

/// Gibraltar Pound
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct GIP;
impl Currency for GIP {
    fn code(&self) -> &'static str {
        "GIP"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        292
    }
}

/// Quetzal
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct GTQ;
impl Currency for GTQ {
    fn code(&self) -> &'static str {
        "GTQ"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        320
    }
}

/// Pound Sterling
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct GBP;
impl Currency for GBP {
    fn code(&self) -> &'static str {
        "GBP"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        826
    }
}

/// Guinean Franc
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct GNF;
impl Currency for GNF {
    fn code(&self) -> &'static str {
        "GNF"
    }

    fn minor_units(&self) -> u32 {
        0
    }

    fn numeric_code(&self) -> u32 {
        324
    }
}

/// Guyana Dollar
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct GYD;
impl Currency for GYD {
    fn code(&self) -> &'static str {
        "GYD"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        328
    }
}

/// Gourde
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct HTG;
impl Currency for HTG {
    fn code(&self) -> &'static str {
        "HTG"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        332
    }
}

/// Lempira
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct HNL;
impl Currency for HNL {
    fn code(&self) -> &'static str {
        "HNL"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        340
    }
}

/// Hong Kong Dollar
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct HKD;
impl Currency for HKD {
    fn code(&self) -> &'static str {
        "HKD"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        344
    }
}

/// Forint
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct HUF;
impl Currency for HUF {
    fn code(&self) -> &'static str {
        "HUF"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        348
    }
}

/// Iceland Krona
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ISK;
impl Currency for ISK {
    fn code(&self) -> &'static str {
        "ISK"
    }

    fn minor_units(&self) -> u32 {
        0
    }

    fn numeric_code(&self) -> u32 {
        352
    }
}

/// Rupiah
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct IDR;
impl Currency for IDR {
    fn code(&self) -> &'static str {
        "IDR"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        360
    }
}

/// SDR (Special Drawing Right)
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct XDR;
impl Currency for XDR {
    fn code(&self) -> &'static str {
        "XDR"
    }

    fn minor_units(&self) -> u32 {
        0
    }

    fn numeric_code(&self) -> u32 {
        960
    }
}

/// Iranian Rial
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct IRR;
impl Currency for IRR {
    fn code(&self) -> &'static str {
        "IRR"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        364
    }
}

/// Iraqi Dinar
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct IQD;
impl Currency for IQD {
    fn code(&self) -> &'static str {
        "IQD"
    }

    fn minor_units(&self) -> u32 {
        3
    }

    fn numeric_code(&self) -> u32 {
        368
    }
}

/// New Israeli Sheqel
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ILS;
impl Currency for ILS {
    fn code(&self) -> &'static str {
        "ILS"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        376
    }
}

/// Jamaican Dollar
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct JMD;
impl Currency for JMD {
    fn code(&self) -> &'static str {
        "JMD"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        388
    }
}

/// Yen
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct JPY;
impl Currency for JPY {
    fn code(&self) -> &'static str {
        "JPY"
    }

    fn minor_units(&self) -> u32 {
        0
    }

    fn numeric_code(&self) -> u32 {
        392
    }
}

/// Jordanian Dinar
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct JOD;
impl Currency for JOD {
    fn code(&self) -> &'static str {
        "JOD"
    }

    fn minor_units(&self) -> u32 {
        3
    }

    fn numeric_code(&self) -> u32 {
        400
    }
}

/// Tenge
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct KZT;
impl Currency for KZT {
    fn code(&self) -> &'static str {
        "KZT"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        398
    }
}

/// Kenyan Shilling
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct KES;
impl Currency for KES {
    fn code(&self) -> &'static str {
        "KES"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        404
    }
}

/// North Korean Won
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct KPW;
impl Currency for KPW {
    fn code(&self) -> &'static str {
        "KPW"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        408
    }
}

/// Won
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct KRW;
impl Currency for KRW {
    fn code(&self) -> &'static str {
        "KRW"
    }

    fn minor_units(&self) -> u32 {
        0
    }

    fn numeric_code(&self) -> u32 {
        410
    }
}

/// Kuwaiti Dinar
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct KWD;
impl Currency for KWD {
    fn code(&self) -> &'static str {
        "KWD"
    }

    fn minor_units(&self) -> u32 {
        3
    }

    fn numeric_code(&self) -> u32 {
        414
    }
}

/// Som
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct KGS;
impl Currency for KGS {
    fn code(&self) -> &'static str {
        "KGS"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        417
    }
}

/// Lao Kip
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct LAK;
impl Currency for LAK {
    fn code(&self) -> &'static str {
        "LAK"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        418
    }
}

/// Lebanese Pound
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct LBP;
impl Currency for LBP {
    fn code(&self) -> &'static str {
        "LBP"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        422
    }
}

/// Loti
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct LSL;
impl Currency for LSL {
    fn code(&self) -> &'static str {
        "LSL"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        426
    }
}

/// Rand
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ZAR;
impl Currency for ZAR {
    fn code(&self) -> &'static str {
        "ZAR"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        710
    }
}

/// Liberian Dollar
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct LRD;
impl Currency for LRD {
    fn code(&self) -> &'static str {
        "LRD"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        430
    }
}

/// Libyan Dinar
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct LYD;
impl Currency for LYD {
    fn code(&self) -> &'static str {
        "LYD"
    }

    fn minor_units(&self) -> u32 {
        3
    }

    fn numeric_code(&self) -> u32 {
        434
    }
}

/// Swiss Franc
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct CHF;
impl Currency for CHF {
    fn code(&self) -> &'static str {
        "CHF"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        756
    }
}

/// Pataca
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MOP;
impl Currency for MOP {
    fn code(&self) -> &'static str {
        "MOP"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        446
    }
}

/// Denar
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MKD;
impl Currency for MKD {
    fn code(&self) -> &'static str {
        "MKD"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        807
    }
}

/// Malagasy Ariary
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MGA;
impl Currency for MGA {
    fn code(&self) -> &'static str {
        "MGA"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        969
    }
}

/// Malawi Kwacha
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MWK;
impl Currency for MWK {
    fn code(&self) -> &'static str {
        "MWK"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        454
    }
}

/// Malaysian Ringgit
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MYR;
impl Currency for MYR {
    fn code(&self) -> &'static str {
        "MYR"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        458
    }
}

/// Rufiyaa
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MVR;
impl Currency for MVR {
    fn code(&self) -> &'static str {
        "MVR"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        462
    }
}

/// Ouguiya
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MRU;
impl Currency for MRU {
    fn code(&self) -> &'static str {
        "MRU"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        929
    }
}

/// Mauritius Rupee
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MUR;
impl Currency for MUR {
    fn code(&self) -> &'static str {
        "MUR"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        480
    }
}

/// ADB Unit of Account
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct XUA;
impl Currency for XUA {
    fn code(&self) -> &'static str {
        "XUA"
    }

    fn minor_units(&self) -> u32 {
        0
    }

    fn numeric_code(&self) -> u32 {
        965
    }
}

/// Mexican Peso
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MXN;
impl Currency for MXN {
    fn code(&self) -> &'static str {
        "MXN"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        484
    }
}

/// Mexican Unidad de Inversion (UDI)
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MXV;
impl Currency for MXV {
    fn code(&self) -> &'static str {
        "MXV"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        979
    }
}

/// Moldovan Leu
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MDL;
impl Currency for MDL {
    fn code(&self) -> &'static str {
        "MDL"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        498
    }
}

/// Tugrik
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MNT;
impl Currency for MNT {
    fn code(&self) -> &'static str {
        "MNT"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        496
    }
}

/// Moroccan Dirham
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MAD;
impl Currency for MAD {
    fn code(&self) -> &'static str {
        "MAD"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        504
    }
}

/// Mozambique Metical
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MZN;
impl Currency for MZN {
    fn code(&self) -> &'static str {
        "MZN"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        943
    }
}

/// Kyat
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MMK;
impl Currency for MMK {
    fn code(&self) -> &'static str {
        "MMK"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        104
    }
}

/// Namibia Dollar
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct NAD;
impl Currency for NAD {
    fn code(&self) -> &'static str {
        "NAD"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        516
    }
}

/// Nepalese Rupee
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct NPR;
impl Currency for NPR {
    fn code(&self) -> &'static str {
        "NPR"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        524
    }
}

/// Cordoba Oro
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct NIO;
impl Currency for NIO {
    fn code(&self) -> &'static str {
        "NIO"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        558
    }
}

/// Naira
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct NGN;
impl Currency for NGN {
    fn code(&self) -> &'static str {
        "NGN"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        566
    }
}

/// Rial Omani
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct OMR;
impl Currency for OMR {
    fn code(&self) -> &'static str {
        "OMR"
    }

    fn minor_units(&self) -> u32 {
        3
    }

    fn numeric_code(&self) -> u32 {
        512
    }
}

/// Pakistan Rupee
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct PKR;
impl Currency for PKR {
    fn code(&self) -> &'static str {
        "PKR"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        586
    }
}

/// Balboa
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct PAB;
impl Currency for PAB {
    fn code(&self) -> &'static str {
        "PAB"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        590
    }
}

/// Kina
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct PGK;
impl Currency for PGK {
    fn code(&self) -> &'static str {
        "PGK"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        598
    }
}

/// Guarani
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct PYG;
impl Currency for PYG {
    fn code(&self) -> &'static str {
        "PYG"
    }

    fn minor_units(&self) -> u32 {
        0
    }

    fn numeric_code(&self) -> u32 {
        600
    }
}

/// Sol
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct PEN;
impl Currency for PEN {
    fn code(&self) -> &'static str {
        "PEN"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        604
    }
}

/// Philippine Peso
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct PHP;
impl Currency for PHP {
    fn code(&self) -> &'static str {
        "PHP"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        608
    }
}

/// Zloty
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct PLN;
impl Currency for PLN {
    fn code(&self) -> &'static str {
        "PLN"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        985
    }
}

/// Qatari Rial
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct QAR;
impl Currency for QAR {
    fn code(&self) -> &'static str {
        "QAR"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        634
    }
}

/// Romanian Leu
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct RON;
impl Currency for RON {
    fn code(&self) -> &'static str {
        "RON"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        946
    }
}

/// Russian Ruble
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct RUB;
impl Currency for RUB {
    fn code(&self) -> &'static str {
        "RUB"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        643
    }
}

/// Rwanda Franc
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct RWF;
impl Currency for RWF {
    fn code(&self) -> &'static str {
        "RWF"
    }

    fn minor_units(&self) -> u32 {
        0
    }

    fn numeric_code(&self) -> u32 {
        646
    }
}

/// Saint Helena Pound
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct SHP;
impl Currency for SHP {
    fn code(&self) -> &'static str {
        "SHP"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        654
    }
}

/// Tala
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct WST;
impl Currency for WST {
    fn code(&self) -> &'static str {
        "WST"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        882
    }
}

/// Dobra
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct STN;
impl Currency for STN {
    fn code(&self) -> &'static str {
        "STN"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        930
    }
}

/// Saudi Riyal
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct SAR;
impl Currency for SAR {
    fn code(&self) -> &'static str {
        "SAR"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        682
    }
}

/// Serbian Dinar
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct RSD;
impl Currency for RSD {
    fn code(&self) -> &'static str {
        "RSD"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        941
    }
}

/// Seychelles Rupee
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct SCR;
impl Currency for SCR {
    fn code(&self) -> &'static str {
        "SCR"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        690
    }
}

/// Leone
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct SLE;
impl Currency for SLE {
    fn code(&self) -> &'static str {
        "SLE"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        925
    }
}

/// Singapore Dollar
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct SGD;
impl Currency for SGD {
    fn code(&self) -> &'static str {
        "SGD"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        702
    }
}

/// Sucre
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct XSU;
impl Currency for XSU {
    fn code(&self) -> &'static str {
        "XSU"
    }

    fn minor_units(&self) -> u32 {
        0
    }

    fn numeric_code(&self) -> u32 {
        994
    }
}

/// Solomon Islands Dollar
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct SBD;
impl Currency for SBD {
    fn code(&self) -> &'static str {
        "SBD"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        90
    }
}

/// Somali Shilling
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct SOS;
impl Currency for SOS {
    fn code(&self) -> &'static str {
        "SOS"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        706
    }
}

/// South Sudanese Pound
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct SSP;
impl Currency for SSP {
    fn code(&self) -> &'static str {
        "SSP"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        728
    }
}

/// Sri Lanka Rupee
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct LKR;
impl Currency for LKR {
    fn code(&self) -> &'static str {
        "LKR"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        144
    }
}

/// Sudanese Pound
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct SDG;
impl Currency for SDG {
    fn code(&self) -> &'static str {
        "SDG"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        938
    }
}

/// Surinam Dollar
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct SRD;
impl Currency for SRD {
    fn code(&self) -> &'static str {
        "SRD"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        968
    }
}

/// Swedish Krona
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct SEK;
impl Currency for SEK {
    fn code(&self) -> &'static str {
        "SEK"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        752
    }
}

/// WIR Euro
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct CHE;
impl Currency for CHE {
    fn code(&self) -> &'static str {
        "CHE"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        947
    }
}

/// WIR Franc
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct CHW;
impl Currency for CHW {
    fn code(&self) -> &'static str {
        "CHW"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        948
    }
}

/// Syrian Pound
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct SYP;
impl Currency for SYP {
    fn code(&self) -> &'static str {
        "SYP"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        760
    }
}

/// New Taiwan Dollar
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct TWD;
impl Currency for TWD {
    fn code(&self) -> &'static str {
        "TWD"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        901
    }
}

/// Somoni
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct TJS;
impl Currency for TJS {
    fn code(&self) -> &'static str {
        "TJS"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        972
    }
}

/// Tanzanian Shilling
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct TZS;
impl Currency for TZS {
    fn code(&self) -> &'static str {
        "TZS"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        834
    }
}

/// Baht
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct THB;
impl Currency for THB {
    fn code(&self) -> &'static str {
        "THB"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        764
    }
}

/// Pa’anga
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct TOP;
impl Currency for TOP {
    fn code(&self) -> &'static str {
        "TOP"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        776
    }
}

/// Trinidad and Tobago Dollar
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct TTD;
impl Currency for TTD {
    fn code(&self) -> &'static str {
        "TTD"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        780
    }
}

/// Tunisian Dinar
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct TND;
impl Currency for TND {
    fn code(&self) -> &'static str {
        "TND"
    }

    fn minor_units(&self) -> u32 {
        3
    }

    fn numeric_code(&self) -> u32 {
        788
    }
}

/// Turkish Lira
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct TRY;
impl Currency for TRY {
    fn code(&self) -> &'static str {
        "TRY"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        949
    }
}

/// Turkmenistan New Manat
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct TMT;
impl Currency for TMT {
    fn code(&self) -> &'static str {
        "TMT"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        934
    }
}

/// Uganda Shilling
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct UGX;
impl Currency for UGX {
    fn code(&self) -> &'static str {
        "UGX"
    }

    fn minor_units(&self) -> u32 {
        0
    }

    fn numeric_code(&self) -> u32 {
        800
    }
}

/// Hryvnia
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct UAH;
impl Currency for UAH {
    fn code(&self) -> &'static str {
        "UAH"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        980
    }
}

/// UAE Dirham
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct AED;
impl Currency for AED {
    fn code(&self) -> &'static str {
        "AED"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        784
    }
}

/// US Dollar (Next day)
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct USN;
impl Currency for USN {
    fn code(&self) -> &'static str {
        "USN"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        997
    }
}

/// Peso Uruguayo
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct UYU;
impl Currency for UYU {
    fn code(&self) -> &'static str {
        "UYU"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        858
    }
}

/// Uruguay Peso en Unidades Indexadas (UI)
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct UYI;
impl Currency for UYI {
    fn code(&self) -> &'static str {
        "UYI"
    }

    fn minor_units(&self) -> u32 {
        0
    }

    fn numeric_code(&self) -> u32 {
        940
    }
}

/// Unidad Previsional
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct UYW;
impl Currency for UYW {
    fn code(&self) -> &'static str {
        "UYW"
    }

    fn minor_units(&self) -> u32 {
        4
    }

    fn numeric_code(&self) -> u32 {
        927
    }
}

/// Uzbekistan Sum
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct UZS;
impl Currency for UZS {
    fn code(&self) -> &'static str {
        "UZS"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        860
    }
}

/// Vatu
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct VUV;
impl Currency for VUV {
    fn code(&self) -> &'static str {
        "VUV"
    }

    fn minor_units(&self) -> u32 {
        0
    }

    fn numeric_code(&self) -> u32 {
        548
    }
}

/// Bolívar Soberano
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct VES;
impl Currency for VES {
    fn code(&self) -> &'static str {
        "VES"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        928
    }
}

/// Bolívar Soberano
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct VED;
impl Currency for VED {
    fn code(&self) -> &'static str {
        "VED"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        926
    }
}

/// Dong
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct VND;
impl Currency for VND {
    fn code(&self) -> &'static str {
        "VND"
    }

    fn minor_units(&self) -> u32 {
        0
    }

    fn numeric_code(&self) -> u32 {
        704
    }
}

/// Yemeni Rial
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct YER;
impl Currency for YER {
    fn code(&self) -> &'static str {
        "YER"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        886
    }
}

/// Zambian Kwacha
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ZMW;
impl Currency for ZMW {
    fn code(&self) -> &'static str {
        "ZMW"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        967
    }
}

/// Zimbabwe Gold
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ZWG;
impl Currency for ZWG {
    fn code(&self) -> &'static str {
        "ZWG"
    }

    fn minor_units(&self) -> u32 {
        2
    }

    fn numeric_code(&self) -> u32 {
        924
    }
}

/// Bond Markets Unit European Composite Unit (EURCO)
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct XBA;
impl Currency for XBA {
    fn code(&self) -> &'static str {
        "XBA"
    }

    fn minor_units(&self) -> u32 {
        0
    }

    fn numeric_code(&self) -> u32 {
        955
    }
}

/// Bond Markets Unit European Monetary Unit (E.M.U.-6)
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct XBB;
impl Currency for XBB {
    fn code(&self) -> &'static str {
        "XBB"
    }

    fn minor_units(&self) -> u32 {
        0
    }

    fn numeric_code(&self) -> u32 {
        956
    }
}

/// Bond Markets Unit European Unit of Account 9 (E.U.A.-9)
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct XBC;
impl Currency for XBC {
    fn code(&self) -> &'static str {
        "XBC"
    }

    fn minor_units(&self) -> u32 {
        0
    }

    fn numeric_code(&self) -> u32 {
        957
    }
}

/// Bond Markets Unit European Unit of Account 17 (E.U.A.-17)
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct XBD;
impl Currency for XBD {
    fn code(&self) -> &'static str {
        "XBD"
    }

    fn minor_units(&self) -> u32 {
        0
    }

    fn numeric_code(&self) -> u32 {
        958
    }
}

/// Codes specifically reserved for testing purposes
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct XTS;
impl Currency for XTS {
    fn code(&self) -> &'static str {
        "XTS"
    }

    fn minor_units(&self) -> u32 {
        0
    }

    fn numeric_code(&self) -> u32 {
        963
    }
}

/// The codes assigned for transactions where no currency is involved
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct XXX;
impl Currency for XXX {
    fn code(&self) -> &'static str {
        "XXX"
    }

    fn minor_units(&self) -> u32 {
        0
    }

    fn numeric_code(&self) -> u32 {
        999
    }
}

/// Gold
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct XAU;
impl Currency for XAU {
    fn code(&self) -> &'static str {
        "XAU"
    }

    fn minor_units(&self) -> u32 {
        0
    }

    fn numeric_code(&self) -> u32 {
        959
    }
}

/// Palladium
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct XPD;
impl Currency for XPD {
    fn code(&self) -> &'static str {
        "XPD"
    }

    fn minor_units(&self) -> u32 {
        0
    }

    fn numeric_code(&self) -> u32 {
        964
    }
}

/// Platinum
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct XPT;
impl Currency for XPT {
    fn code(&self) -> &'static str {
        "XPT"
    }

    fn minor_units(&self) -> u32 {
        0
    }

    fn numeric_code(&self) -> u32 {
        962
    }
}

/// Silver
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct XAG;
impl Currency for XAG {
    fn code(&self) -> &'static str {
        "XAG"
    }

    fn minor_units(&self) -> u32 {
        0
    }

    fn numeric_code(&self) -> u32 {
        961
    }
}
