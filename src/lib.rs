mod aux;
mod hexes;
pub use hexes::{Hexes, HexIterator};

#[cfg(test)]
mod tests {
    use crate::hexes::HexIterator;

    #[test]
    fn hexes_iterates_over_an_array() {
        let data = [0xbe_u8, 0xef_u8];
        let it = data.iter().copied().hexed();

        let collected: String = it.collect();
        assert_eq!(collected.as_str(), "beef");
    }

    #[test]
    fn hexes_reverse_iterates_over_an_array() {
        let data = [0xbe_u8, 0xef_u8];
        let it = data.iter().copied().hexed().rev();

        let collected: String = it.collect();
        assert_eq!(collected.as_str(), "feeb");
    }

    #[test]
    fn hexes_transforms_ranges() {
        let range = 10..16;
        let it = range.hexed();

        let collected: String = it.collect();
        assert_eq!(collected.as_str(), "0a0b0c0d0e0f");
    }

    #[test]
    fn hexes_works_for_converted_strings() {
        let data = b"beef";
        let it = data.iter().copied().hexed();

        let collected: String = it.collect();
        assert_eq!(collected.as_str(), "62656566");
    }
}
