pub use self::mouse::Mouse;

/// Tracked state related to the mouse.
pub mod mouse {
    use math::Point2;
    use std;
    use window;

    #[doc(inline)]
    pub use event::MouseButton as Button;

    /// The max total number of buttons on a mouse.
    pub const NUM_BUTTONS: usize = 9;

    /// The state of the `Mouse` at a single moment in time.
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct Mouse {
        /// The ID of the last window currently in focus.
        pub window: Option<window::Id>,
        /// *x* position relative to the middle of `window`.
        pub x: f64,
        /// *y* position relative to the middle of `window`.
        pub y: f64,
        /// A map describing the state of each mouse button.
        pub buttons: ButtonMap,
    }

    /// Whether the button is up or down.
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub enum ButtonPosition {
        /// The button is up (i.e. pressed).
        Up,
        /// The button is down and was originally pressed down at the given `Point2`.
        Down(Point2<f64>),
    }

    /// Stores the state of all mouse buttons.
    ///
    /// If the mouse button is down, it stores the position of the mouse when the button was pressed
    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct ButtonMap {
        buttons: [ButtonPosition; NUM_BUTTONS],
    }

    /// An iterator yielding all pressed buttons.
    #[derive(Clone)]
    pub struct PressedButtons<'a> {
        buttons: std::iter::Enumerate<std::slice::Iter<'a, ButtonPosition>>,
    }

    impl Mouse {
        /// Construct a new default `Mouse`.
        pub fn new() -> Self {
            Mouse {
                window: None,
                buttons: ButtonMap::new(),
                x: 0.0,
                y: 0.0,
            }
        }

        /// The position of the mouse relative to the middle of the window in focus..
        pub fn position(&self) -> Point2<f64> {
            Point2 { x: self.x, y: self.y }
        }
    }

    impl ButtonPosition {
        /// If the mouse button is down, return a new one with position relative to the given `xy`.
        pub fn relative_to(self, xy: Point2<f64>) -> Self {
            match self {
                ButtonPosition::Down(pos) => {
                    let rel_p = pos - xy;
                    ButtonPosition::Down(Point2 { x: rel_p.x, y: rel_p.y })
                }
                button_pos => button_pos,
            }
        }

        /// Is the `ButtonPosition` down.
        pub fn is_down(&self) -> bool {
            match *self {
                ButtonPosition::Down(_) => true,
                _ => false,
            }
        }

        /// Is the `ButtonPosition` up.
        pub fn is_up(&self) -> bool {
            match *self {
                ButtonPosition::Up => true,
                _ => false,
            }
        }

        /// Returns the position at which the button was pressed.
        pub fn if_down(&self) -> Option<Point2<f64>> {
            match *self {
                ButtonPosition::Down(xy) => Some((xy)),
                _ => None,
            }
        }
    }

    impl ButtonMap {
        /// Returns a new button map with all states set to `None`
        pub fn new() -> Self {
            ButtonMap { buttons: [ButtonPosition::Up; NUM_BUTTONS] }
        }

        /// Returns a copy of the ButtonMap relative to the given `Point`
        pub fn relative_to(self, xy: Point2<f64>) -> Self {
            self.buttons.iter().enumerate().fold(
                ButtonMap::new(),
                |mut map,
                 (idx, button_pos)| {
                    map.buttons[idx] = button_pos.relative_to(xy);
                    map
                },
            )
        }

        /// The state of the left mouse button.
        pub fn left(&self) -> &ButtonPosition {
            &self[Button::Left]
        }

        /// The state of the middle mouse button.
        pub fn middle(&self) -> &ButtonPosition {
            &self[Button::Middle]
        }

        /// The state of the right mouse button.
        pub fn right(&self) -> &ButtonPosition {
            &self[Button::Right]
        }

        /// Sets the `Button` in the `Down` position.
        pub fn press(&mut self, button: Button, xy: Point2<f64>) {
            self.buttons[button_to_idx(button)] = ButtonPosition::Down(xy);
        }

        /// Set's the `Button` in the `Up` position.
        pub fn release(&mut self, button: Button) {
            self.buttons[button_to_idx(button)] = ButtonPosition::Up;
        }

        /// An iterator yielding all pressed mouse buttons along with the location at which they
        /// were originally pressed.
        pub fn pressed(&self) -> PressedButtons {
            PressedButtons { buttons: self.buttons.iter().enumerate() }
        }
    }

    impl std::ops::Index<Button> for ButtonMap {
        type Output = ButtonPosition;
        fn index(&self, button: Button) -> &Self::Output {
            &self.buttons[button_to_idx(button)]
        }
    }

    impl<'a> Iterator for PressedButtons<'a> {
        type Item = (Button, Point2<f64>);
        fn next(&mut self) -> Option<Self::Item> {
            while let Some((idx, button_pos)) = self.buttons.next() {
                if let ButtonPosition::Down(xy) = *button_pos {
                    return Some((idx_to_button(idx), xy));
                }
            }
            None
        }
    }

    fn idx_to_button(i: usize) -> Button {
        match i {
            n @ 0...5 => Button::Other(n as u8),
            6 => Button::Left,
            7 => Button::Right,
            8 => Button::Middle,
            _ => Button::Other(std::u8::MAX),
        }
    }

    fn button_to_idx(button: Button) -> usize {
        match button {
            Button::Other(n) => n as usize,
            Button::Left => 6,
            Button::Right => 7,
            Button::Middle => 8,
        }
    }

}
