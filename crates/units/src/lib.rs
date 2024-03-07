pub use unc_units_core::*;
pub use unc_units_macro::*;

#[cfg(test)]
mod tests {
    use super::*;
    const ONE_UNC: u128 = 1_000_000_000_000_000_000_000_000;
    const ONE_TGAS: u128 = 1_000_000_000_000;
    const ONE_PARSED_UNC: u128 = parse_unc!("1 U");
    const ONE_PARSED_TGAS: u128 = parse_gas!("1 TGas");

    #[test]
    fn const_parse_unc() {
        assert_eq!(near::parse("1U").unwrap(), ONE_UNC);
        assert_eq!(ONE_PARSED_UNC, ONE_UNC);
    }

    #[test]
    fn const_parse_gas() {
        assert_eq!(gas::parse("1 TGas").unwrap(), ONE_TGAS);
        assert_eq!(ONE_PARSED_TGAS, ONE_TGAS);
    }
}
