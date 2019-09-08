mod hexes;
pub use hexes::{Hexes, hexes};

#[cfg(test)]
mod tests {
    use crate::hexes::hexes;

    #[test]
    fn hexes_iterates_over_byte_slice() {
        let data= &[0xbe_u8, 0xef_u8];
        let it = hexes(data);

        let collected_hexes: String = it.collect();
        assert_eq!(collected_hexes.as_str(), "beef")
    }

    #[test]
    fn hexes_reverse_iterates_over_byte_slice() {
        let data = &[0xbe_u8, 0xef_u8];
        let it = hexes(data).rev();

        let collected_hexes: String = it.collect();
        assert_eq!(collected_hexes.as_str(), "feeb");
    }

    #[test]
    fn hexes_works_for_converted_strings() {
        let data = b"beef";
        let it = hexes(data);

        let collected_hexes: String = it.collect();
        assert_eq!(collected_hexes.as_str(), "62656566");
    }
}
