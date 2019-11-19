
#[derive(PartialEq, Debug)]
pub struct BRL(i32);
#[derive(PartialEq, Debug)]
pub struct USD(i32);
#[derive(PartialEq, Debug)]
pub struct GBP(i32);
#[derive(PartialEq, Debug)]
pub struct CAD(i32);

pub trait FromUsd<T> {
    fn from_value(&self, value: f32) -> T;
}

pub trait ToUsd<T> {
    fn to_value(&self, value: T) -> f32;
}

pub struct Ex {
    cad: f32,
    gbp: f32
}

impl ToUsd<GBP> for Ex {
    fn to_value(&self, value: GBP) -> f32 {
        (value.0 as f32) * self.gbp
    }
}

impl ToUsd<CAD> for Ex {
    fn to_value(&self, value: CAD) -> f32 {
        (value.0 as f32) * self.cad
    }
}

impl FromUsd<CAD> for Ex {
    fn from_value(&self, value: f32) -> CAD {
        CAD((value / self.cad) as i32)
    }
}

impl FromUsd<GBP> for Ex {
    fn from_value(&self, value: f32) -> GBP {
        GBP((value / self.gbp) as i32)
    } 
}

pub trait Exchange<E, T> {
    fn convert(&self, from: E) -> T;
}

impl<E, F, T> Exchange<F, T> 
    for E where E : ToUsd<F> + FromUsd<T> {
    fn convert(&self, from: F) -> T {
        self.from_value(self.to_value(from))
    } 
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let gbp = GBP(200);
        let exchange = Ex {
            cad: 0.7,
            gbp: 1.3
        };

        let cad: CAD = exchange.convert(gbp);
        assert_eq!(cad, CAD(371));
    }
}