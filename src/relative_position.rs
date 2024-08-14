use crate::rect::Rect;

/// This trait defines methods for determining the relative position of one
/// rectangle compared to another.
pub trait RelativePosition {
    /// Checks if the calling rectangle is to the left of the other rectangle.
    fn left_of(&self, other: &Self) -> bool;

    /// Checks if the calling rectangle is to the right of the other rectangle.
    fn right_of(&self, other: &Self) -> bool;

    /// Checks if the calling rectangle is above the other rectangle.
    fn above(&self, other: &Self) -> bool;

    /// Checks if the calling rectangle is below the other rectangle.
    fn below(&self, other: &Self) -> bool;
}

impl RelativePosition for Rect {
    fn left_of(&self, other: &Self) -> bool {
        self.right() < other.left()
    }

    fn right_of(&self, other: &Self) -> bool {
        self.left() > other.right()
    }

    fn above(&self, other: &Self) -> bool {
        self.bottom() < other.top()
    }

    fn below(&self, other: &Self) -> bool {
        self.top() > other.bottom()
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use super::*;

    #[rstest]
    #[case::left_of((100, 100, 50, 50), (200, 100, 50, 50), Rect::left_of)]
    #[case::right_of((200, 100, 50, 50), (100, 100, 50, 50), Rect::right_of)]
    #[case::above((100, 100, 50, 50), (100, 200, 50, 50), Rect::above)]
    #[case::below((100, 200, 50, 50), (100, 100, 50, 50), Rect::below)]
    fn test_positions(
        #[case] rect1: (i32, i32, i32, i32),
        #[case] rect2: (i32, i32, i32, i32),
        #[case] orientation: fn(&Rect, &Rect) -> bool
    ) {
        let rect1 = Rect::from(rect1);
        let rect2 = Rect::from(rect2);
        assert!(orientation(&rect1, &rect2));
    }
}
