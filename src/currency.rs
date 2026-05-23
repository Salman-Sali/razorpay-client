use strum::Display;
use strum_macros::EnumIter;


#[derive(serde::Serialize, serde::Deserialize, Display,EnumIter, Debug, Clone)]
pub enum Currency {
    //United Arab Emirates Dirham
    AED,
    //Albanian lek
    ALL,
    //Armenian dram
    AMD,
    //Australian dollar
    AUD,
    //Aruban florin
    AWG,
    //Azerbaijan Manat
    AZN,
    //Convertible Mark
    BAM,
    //Barbadian dollar
    BBD,
    //Bangladeshi taka
    BDT,
    //Bulgarian Lev
    BGN,
    //Bahraini Dinar
    BHD,
    //Burundi Franc
    BIF,
    //Bermudian dollar
    BMD,
    //Brunei dollar
    BND,
    //Bolivian boliviano
    BOB,
    //Brazilian Real
    BRL,
    //Bahamian dollar
    BSD,
    //Bhutanese Ngultrum
    BTN,
    //Botswana pula
    BWP,
    //Belize dollar
    BZD,
    //Canadian dollar
    CAD,
    //Swiss franc
    CHF,
    //Chilean Peso
    CLP,
    //Chinese yuan renminbi
    CNY,
    //Colombian peso
    COP,
    //Costa Rican colon
    CRC,
    //Cuban peso
    CUP,
    //Cabo Verde Escudo
    CVE,
    //Czech koruna
    CZK,
    //Djibouti Franc
    DJF,
    //Danish krone
    DKK,
    //Dominican peso
    DOP,
    //Algerian dinar
    DZD,
    //Egyptian pound
    EGP,
    //Ethiopian birr
    ETB,
    //European euro
    EUR,
    //Fijian dollar
    FJD,
    //Pound sterling
    GBP,
    //Ghanian Cedi
    GHS,
    //Gibraltar pound
    GIP,
    //Gambian dalasi
    GMD,
    //Guinean Franc
    GNF,
    //Guatemalan quetzal
    GTQ,
    //Guyanese dollar
    GYD,
    //Hong Kong dollar
    HKD,
    //Honduran lempira
    HNL,
    //Croatian kuna
    HRK,
    //Haitian gourde
    HTG,
    //Hungarian forint
    HUF,
    //Indonesian rupiah
    IDR,
    //Israeli new shekel
    ILS,
    //Indian rupee
    INR,
    //Iraqi Dinar
    IQD,
    //Iceland Krona
    ISK,
    //Jamaican dollar
    JMD,
    //Jordanian Dinar
    JOD,
    //Japanese Yen
    JPY,
    //Kenyan shilling
    KES,
    //Kyrgyzstani som
    KGS,
    //Cambodian riel
    KHR,
    //Comorian Franc
    KMF,
    //Korean Won
    KRW,
    //Kuwaiti Dinar
    KWD,
    //Cayman Islands dollar
    KYD,
    //Kazakhstani tenge
    KZT,
    //Lao kip
    LAK,
    //Sri Lankan rupee
    LKR,
    //Liberian dollar
    LRD,
    //Lesotho loti
    LSL,
    //Moroccan dirham
    MAD,
    //Moldovan leu
    MDL,
    //Malagasy Ariary
    MGA,
    //Macedonian denar
    MKD,
    //Myanmar kyat
    MMK,
    //Mongolian tugrik
    MNT,
    //Macanese pataca
    MOP,
    //Mauritian rupee
    MUR,
    //Maldivian rufiyaa
    MVR,
    //Malawian kwacha
    MWK,
    //Mexican peso
    MXN,
    //Malaysian ringgit
    MYR,
    //Mozambique Metical
    MZN,
    //Namibian dollar
    NAD,
    //Nigerian naira
    NGN,
    //Nicaraguan cordoba
    NIO,
    //Norwegian krone
    NOK,
    //Nepalese rupee
    NPR,
    //New Zealand dollar
    NZD,
    //Rial Omani
    OMR,
    //Peruvian sol
    PEN,
    //Papua New Guinean kina
    PGK,
    //Philippine peso
    PHP,
    //Pakistani rupee
    PKR,
    //Polish Zloty
    PLN,
    //Paraguayan Guarani
    PYG,
    //Qatari riyal
    QAR,
    //Romanian Leu
    RON,
    //Serbian Dinar
    RSD,
    //Russian ruble
    RUB,
    //Rwanda Franc
    RWF,
    //Saudi Arabian riyal
    SAR,
    //Seychellois rupee
    SCR,
    //Swedish krona
    SEK,
    //Singapore dollar
    SGD,
    //Sierra Leonean leone
    SLL,
    //Somali shilling
    SOS,
    //Salvadoran colón
    SVC,
    //Swazi lilangeni
    SZL,
    //Thai baht
    THB,
    //Tunisian Dinar
    TND,
    //Turkish Lira
    TRY,
    //Trinidad and Tobago dollar
    TTD,
    //New Taiwan Dollar
    TWD,
    //Tanzanian shilling
    TZS,
    //Ukrainian Hryvnia
    UAH,
    //Uganda Shilling
    UGX,
    //United States dollar
    USD,
    //Uruguayan peso
    UYU,
    //Uzbekistani so'm
    UZS,
    //Vietnamese Dong
    VND,
    //Vatu
    VUV,
    //CFA Franc BEAC
    XAF,
    //East Caribbean Dollar
    XCD,
    //CFA Franc BCEAO
    XOF,
    //CFP Franc
    XPF,
    //Yemeni rial
    YER,
    //South African rand
    ZAR,
    //Zambian Kwacha
    ZMW,
}
