use std::fmt;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Complex<ComponentType> {
    pub real_component: ComponentType,
    pub imaginary_component: ComponentType,
}

impl<ComponentType> Complex<ComponentType> {
    pub fn new(
        real_component: ComponentType,
        imaginary_component: ComponentType,
    ) -> Complex<ComponentType> {
        Complex {
            real_component,
            imaginary_component,
        }
    }
}

// TODO: can make it so it works with different types, but why?
impl<ComponentType> Add<Complex<ComponentType>> for Complex<ComponentType>
where
    ComponentType: Add<Output = ComponentType>,
{
    type Output = Complex<ComponentType>;

    fn add(self, right_hand_side: Complex<ComponentType>) -> Self::Output {
        Complex {
            real_component: self.real_component + right_hand_side.real_component,
            imaginary_component: self.imaginary_component + right_hand_side.imaginary_component,
        }
    }
}

impl<ComponentType> Mul<Complex<ComponentType>> for Complex<ComponentType>
where
    ComponentType: Add<Output = ComponentType>
        + Mul<Output = ComponentType>
        + Sub<Output = ComponentType>
        + Copy,
{
    type Output = Complex<ComponentType>;

    /**
     * (a + bi)(c + di) = ac + adi + bic - bd = (ac - bd) + (ad + bc)i
     */
    fn mul(self, right_hand_side: Complex<ComponentType>) -> Self::Output {
        Complex {
            real_component: self.real_component * right_hand_side.real_component
                - self.imaginary_component * right_hand_side.imaginary_component,
            imaginary_component: self.real_component * right_hand_side.imaginary_component
                + self.imaginary_component * right_hand_side.real_component,
        }
    }
}

// additional math stuff
impl<ComponentType> Complex<ComponentType>
where
    ComponentType: Add<Output = ComponentType>
        + Mul<Output = ComponentType>
        + Sub<Output = ComponentType>
        + Copy,
{
    /**
     * Yes, I know ab+ba is 2ab and I can do other stuff.
     * I am just trying not to do scalar multiplication.
     *
     * Wait, actually tho, why am I doing this?
     *
     * I guess it makes it neater or something
     */
    pub fn square(self) -> Complex<ComponentType> {
        self * self
    }

    pub fn distance_sqrd(self) -> ComponentType {
        self.real_component * self.real_component
            + self.imaginary_component * self.imaginary_component
    }
}

impl<ComponentType> fmt::Display for Complex<ComponentType>
where
    ComponentType: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real_component, self.imaginary_component)
    }
}

#[cfg(test)]
mod tests {
    use super::Complex;

    #[test]
    fn addition_tests() {
        let a = Complex::new(1, 2);
        let b = Complex::new(-4, 9);

        assert!(a + b == Complex::new(-3, 11));
        assert!(b + a == Complex::new(-3, 11));

        let a = Complex::new(1., 2.);
        let b = Complex::new(-4., 9.);
        assert!(a + b == Complex::new(-3., 11.));
        assert!(b + a == Complex::new(-3., 11.));
    }

    #[test]
    fn multiplication_tests() {
        let a = Complex::new(1, 2);
        let b = Complex::new(-4, 9);

        assert!(b * a == Complex::new(-22, 1));
        assert!(a * b == Complex::new(-22, 1));

        let a = Complex::new(1., 2.);
        let b = Complex::new(-4., 9.);
        assert!(a * b == Complex::new(-22., 1.));
        assert!(b * a == Complex::new(-22., 1.));
    }
}
