use std::ops::Add;

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Self;

    fn add(self, other: Meters) -> Self::Output {
        Self(self.0 + other.0 * 1000)
    }
}
