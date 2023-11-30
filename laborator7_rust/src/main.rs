use std::fmt;

#[derive(PartialEq, Debug, Copy, Clone)]
struct Complex
{
    real: f64,
    imag: f64
}

impl Complex
{
    fn new<R, I>(real: R, imag: I) -> Complex
    where
        R: Into<f64>,
        I: Into<f64>,
        {
            return Complex
            {
                real: real.into(),
                imag: imag.into()
            }
        }
    fn conjugate(self) -> Complex
    {
        return Complex
        {
            real: self.real,
            imag: -self.imag
        }
    }
}

impl std::ops::Add<Complex> for Complex
{
    type Output = Complex;
    
    fn add(self, rhs: Complex) -> Self::Output
    {
        return Complex
        {
            real: self.real + rhs.real,
            imag: self.imag + rhs.imag
        }
    }
}

impl std::ops::Sub<Complex> for Complex
{
    type Output = Complex;

    fn sub(self, rhs: Complex) -> Self::Output
    {
        return Complex
        {
            real: self.real - rhs.real,
            imag: self.imag - rhs.imag
        }
    }
}

impl std::ops::Mul<Complex> for Complex
{
    type Output = Complex;

    fn mul(self, rhs: Complex) -> Self::Output
    {
        return Complex
        {
            real: self.real * rhs.real - self.imag * rhs.imag,
            imag: self.real * rhs.imag + self.imag * rhs.real
        }
    }
}

impl std::ops::Neg for Complex
{
    type Output = Complex;

    fn neg(self) -> Self::Output
    {
        return Complex
        {
            real: -self.real,
            imag: -self.imag
        }
    }
}

impl fmt::Display for Complex
{
    fn fmt(&self, format: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        if self.imag == 0.0
        {
            write!(format, "{}", self.real)
        }
        else if self.imag < 0.0
        {
            write!(format, "{}{}i", self.real, self.imag)
        }
        else if self.real == 0.0
        {
            write!(format, "{}i", self.imag)
        }
        else
        {
            write!(format, "{}+{}i", self.real, self.imag)
        }
    }
}

impl std::ops::Mul<f64> for Complex
{
    type Output = Complex;

    fn mul(self, rhs: f64) -> Self::Output
    {
        return Complex
        {
            real: self.real * rhs,
            imag: self.imag * rhs,
        }
    }
}

impl std::ops::Add<i32> for Complex
{
    type Output = Complex;

    fn add(self, rhs: i32) -> Self::Output
    {
        return Complex
        {
            real: self.real + rhs as f64,
            imag: self.imag,
        }
    }
}



fn eq_rel(x: f64, y: f64) -> bool
{
    return (x - y).abs() < 0.001;
}

macro_rules! assert_eq_rel {
    ($x:expr, $y: expr) => {
        let x = $x as f64;
        let y = $y as f64;
        let r = eq_rel(x, y);
        assert!(r, "{} != {}", x, y);
    };
}



fn main() {
    let a = Complex::new(1.0, 2.0);
    assert_eq_rel!(a.real, 1);
    assert_eq_rel!(a.imag, 2);

    let b = Complex::new(2.0, 3);
    let c = a + b;
    assert_eq_rel!(c.real, 3);
    assert_eq_rel!(c.imag, 5);

    let d = c - a;
    assert_eq!(b, d);
    let e = (a * d).conjugate();
    assert_eq_rel!(e.imag, -7);

    let f = (a + b - d) * c;
    assert_eq!(f, Complex::new(-7, 11));

    assert_eq!(Complex::new(1, 2).to_string(), "1+2i");
    assert_eq!(Complex::new(1, -2).to_string(), "1-2i");
    assert_eq!(Complex::new(0, 5).to_string(), "5i");
    assert_eq!(Complex::new(7, 0).to_string(), "7");
    assert_eq!(Complex::new(0, 0).to_string(), "0");

    let h = Complex::new(-4, -5);
    let i = h - (h + 5) * 2.0;
    assert_eq_rel!(i.real, -6);

    let j = -i + i;
    assert_eq_rel!(j.real, 0);
    assert_eq_rel!(j.imag, 0);

    println!("ok!");
}

