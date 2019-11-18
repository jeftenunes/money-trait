
#[derive(PartialEq, Debug)]
pub struct BRL(i32);
#[derive(PartialEq, Debug)]
pub struct USD(i32);
#[derive(PartialEq, Debug)]
pub struct GBP(i32);
#[derive(PartialEq, Debug)]
pub struct CAD(i32);

pub trait ToUSD {
    fn to_usd(&self) -> USD;
    fn convert<T : FromUSD>(&self) -> T {
        T::from_usd(&self.to_usd())
    }
}

impl ToUSD for GBP {
    fn to_usd(&self) -> USD {
        USD((self.0 * 130) / 100)
    }
}

pub trait FromUSD {
    fn from_usd(u:&USD) -> Self;
}

impl FromUSD for CAD {
    fn from_usd(u: &USD) -> Self {
        CAD((u.0 * 130) / 100)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let gbp = GBP(200);
        let usd = gbp.to_usd();

        let cad = CAD::from_usd(&USD(10));

        assert_eq!(usd, USD(260));
        assert_eq!(cad, CAD(13));
    }
}