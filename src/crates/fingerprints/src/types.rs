#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OutputType {
    P2pkh,
    P2sh,
    P2wpkh,
    P2wsh,
    P2tr,
    OpReturn,
    NonStandard,
    // TODO: pay2anchor
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputSortingType {
    Single,
    Ascending,
    Descending,
    Bip69,
    Historical,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputStructureType {
    Single,
    Double,
    Multi,
    Bip69,
}
