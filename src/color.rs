use crate::vec3;
use std::fmt;

pub struct Color(pub vec3::Vec3);

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {}\n",
            (self.0.x * 255.999) as i32,
            (self.0.y * 255.999) as i32,
            (self.0.z * 255.999) as i32
        )
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(
            format!(
                "{}",
                Color(vec3::Vec3 {
                    x: 1.,
                    y: 1.,
                    z: 1.
                })
            ),
            "255 255 255\n"
        );
    }
}
