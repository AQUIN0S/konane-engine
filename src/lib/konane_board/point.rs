#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Piece {
    EMPTY,
    WHITE,
    BLACK,
}

impl Default for Piece {
    fn default() -> Self {
        Piece::EMPTY
    }
}

#[cfg(test)]
mod tests {
    use crate::Piece;

    #[test]
    fn equality_checks() {
        let empty1 = Piece::EMPTY;
        let empty2 = Piece::EMPTY;

        let white1 = Piece::WHITE;
        let white2 = Piece::WHITE;

        let black1 = Piece::BLACK;
        let black2 = Piece::BLACK;

        assert_eq!(empty1, empty2);
        assert_eq!(white1, white2);
        assert_eq!(black1, black2);

        assert_ne!(empty1, white1);
        assert_ne!(empty1, black1);
        assert_ne!(white1, black1);
    }
}
