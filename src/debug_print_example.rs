use std::fmt::{Display, Formatter};

#[allow(dead_code)]
#[derive(Debug)]
struct DebugPrintable(i32);

impl Display for DebugPrintable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "-> DebugPrintable({})", self.0)
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl Display for Complex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

#[cfg(test)]
mod tests {
    use crate::debug_print_example::{Complex, DebugPrintable};

    #[test]
    fn test(){
        let printable = DebugPrintable(12);
        println!("Display: {}", printable);
        println!("DebugPrintable: {:#?}", printable);

        let complex = Complex{
            real: 3.3,
            imag: 7.2,
        };
        println!("Display: {}", complex);
        println!("Debug: {:#?}", complex);
    }
}