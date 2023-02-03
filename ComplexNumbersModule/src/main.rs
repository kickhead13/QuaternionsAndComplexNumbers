use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use core::ops::Add;
use core::ops::Sub;
use std::ops::Mul;

pub fn absolute(x: f64) -> f64 {
    return if x >= 0.0 {x} else {-x};
}

//COMPLEX START
pub struct ComplexNumber {
    pub Re: f64,
    pub Im: f64
}

impl PartialEq for ComplexNumber {
    fn eq(&self, other: &ComplexNumber) -> bool {
        return (self.Re == other.Re)&&(self.Im == other.Im);
    }
}

impl Add<ComplexNumber> for ComplexNumber {
    type Output = ComplexNumber;
    fn add(self, other: ComplexNumber) -> ComplexNumber {
     ComplexNumber {
            Re: self.Re + other.Re,
            Im: self.Im + other.Im,
        }
    }
}

impl Sub<ComplexNumber> for ComplexNumber {
    type Output = ComplexNumber;
    fn sub(self, other: ComplexNumber) -> ComplexNumber {
     ComplexNumber {
            Re: self.Re - other.Re,
            Im: self.Im - other.Im,
        }
    }
}

impl Mul<ComplexNumber> for ComplexNumber {
    type Output = ComplexNumber;
    fn mul(self, other: ComplexNumber) -> ComplexNumber {
        ComplexNumber {
            Re: self.Re*other.Re - self.Im*other.Im,
            Im: self.Re*other.Im + self.Im*other.Re
        }
    }
}

impl Display for ComplexNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if self.Im < 0.0 {
            return if self.Re != 0.0 {write!(f, "{}-{}i", self.Re, absolute(self.Im))}
                    else {write!(f, "{}i",self.Im)};
        }
        if self.Im == 0.0 {
            return write!(f, "{}", self.Re);
        }
        return if self.Re != 0.0 { write!(f, "{}+{}i", self.Re, self.Im)}
                else {write!(f, "{}i", self.Im)}
    }
}

impl ComplexNumber {
    
    fn new() -> Self {
        return Self{
            Re: 0.0,
            Im: 0.0
        }
    }

    fn i() -> Self {
        return Self {
            Re: 0.0,
            Im: 1.0
        }
    }

    fn one() -> Self {
        return Self {
            Re: 1.0,
            Im: 0.0
        }
    }

    fn max() -> Self {
        return Self {
            Re: f64::MAX,
            Im: f64::MAX
        }
    }
    
    fn real(&self) -> bool {
        return self.Im != 0.0;
    }

    fn conjugate(&self) -> Self {
        return Self {
            Re: self.Re,
            Im: -self.Im
        }
    }

    fn range(&self) -> f64 {
        return (self.Re.powi(2) + self.Im.powi(2)).sqrt();
    }

    fn add(&self, other: Self) -> Self {
        return Self {
            Re: self.Re + other.Re,
            Im: self.Im + other.Im
        }
    }

    fn mul(&self, other: Self) -> Self {
        //(a+bi)(c+di) = (ac - bd) + (ad + bc)
        return Self {
            Re: self.Re*other.Re - self.Im*other.Im,
            Im: self.Re*other.Im + self.Im*other.Re
        }
    }

    fn div(&self, other: Self) -> Option<ComplexNumber> {
        //(a+bi)/(c+di) = ((a+bi)*(c-di))/(c*c + d*d)
        if other != ComplexNumber::new() {
            let mut temp: Self = self.mul(other.conjugate());
            temp.apply(|x: f64| {x/(other.Re.powi(2) + &other.Im.powi(2))});
            return Some(Self {
                Re: temp.Re,
                Im: temp.Im
            })
        }
        return None;
    }

    fn apply(&self, f: impl Fn(f64) -> f64) -> Self {
        return Self { 
            Re: f(self.Re),
            Im: f(self.Im)
        }
    }

    fn cprint(&self) {
        println!("{} {}", self.Re, self.Im);
    }

    fn angle(&self) {
        todo!()
    }

}

macro_rules! complex {
    ($ex: expr, $ex2: expr) => {
        ComplexNumber {
            Re: $ex as f64,
            Im: $ex2 as f64
        }
    }
}
//COMPLEX END

//QUATERNIONS START
pub struct Quaternion {
    Re: f64,
    Im: f64,
    Jm: f64,
    Km: f64,
}

macro_rules! quaternion {
    ($ex1:expr, $ex2:expr, $ex3:expr, $ex4:expr) => {
        Quaternion {
            Re: $ex1 as f64,
            Im: $ex2 as f64,
            Jm: $ex3 as f64,
            Km: $ex4 as f64,
        }
    }
}

impl Add<Quaternion> for Quaternion {
    type Output = Quaternion;
    fn add(self, other: Quaternion) -> Quaternion {
        return Quaternion {
            Re: self.Re + other.Re,
            Im: self.Im + other.Im,
            Jm: self.Jm + other.Jm,
            Km: self.Km + other.Km,
        }
    }
}

impl Sub<Quaternion> for Quaternion {
    type Output = Quaternion;
    fn sub(self, other: Quaternion) -> Quaternion {
        return Quaternion {
            Re: self.Re - other.Re,
            Im: self.Im - other.Im,
            Jm: self.Jm - other.Jm,
            Km: self.Km - other.Km,
        }
    }
}

impl Mul<Quaternion> for Quaternion {
    type Output = Quaternion;
    fn mul(self, other: Quaternion) -> Quaternion {
        return Quaternion {
            Re: self.Re * other.Re - self.Im * other.Im - self.Jm * other.Jm - self.Km * other.Km,
            Im: self.Re * other.Im + self.Im * other.Re + self.Jm * other.Km - self.Km * other.Jm,
            Jm: self.Re * other.Jm - self.Im * other.Km + self.Jm * other.Re + self.Km * other.Im,
            Km: self.Re * other.Km + self.Im * other.Jm - self.Jm * other.Im + self.Km * other.Re
        }
    }
}

impl Copy for Quaternion {}

impl Clone for Quaternion {
    fn clone(&self) -> Quaternion {
        *self
    }
}

impl Display for Quaternion {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut text: String = format!("");
        if self.Re != 0.0 {
            text = format!("{}", self.Re);
        }
        if self.Im != 0.0 {
            if self.Im == 1.0 || self.Im == -1.0  {
                text = format!("{}{}i", text,
                        if self.Im == absolute(self.Im) {
                            if self.Re != 0.0 {"+"}else {""}}
                        else {"-"});
            }
            else {
                text = format!("{}{}{}i", text,
                        if self.Im == absolute(self.Im) {"+"} else {"-"},
                        absolute(self.Im));
            }
        }
        if self.Jm != 0.0 {
            if self.Jm == 1.0 || self.Jm == -1.0 {
                text = format!("{}{}j", text,
                        if self.Jm == absolute(self.Jm) {
                            if self.Im != 0.0 || self.Re != 0.0 {"+"}else {""}}
                        else {"-"});
            }
            else {
                text = format!("{}{}{}j", text,
                        if self.Jm == absolute(self.Jm) {"+"} else {"-"},
                        absolute(self.Jm));
            }
        }
        if self.Km != 0.0 {
            if self.Km == 1.0 || self.Km == -1.0 {
                text = format!("{}{}k", text,
                        if self.Km == absolute(self.Km) {
                            if self.Im != 0.0 || self.Re != 0.0 || self.Jm != 0.0 {"+"}else {""}}
                           else {"-"});
            }
            else {
                text = format!("{}{}{}j", text,
                        if self.Km == absolute(self.Km) {"+"} else {"-"},
                        absolute(self.Km));
            }
        }
        return write!(f, "{}", text);
        //return write!(f,"{}{}{}i{}{}j{}{}k", self.Re, if self.Im == absolute(self.Im) {"+"} else {"-"},
         //                                           absolute(self.Im), 
         //                                           if self.Jm == absolute(self.Jm) {"+"} else {"-"},
         //                                           absolute(self.Jm), 
         //                                           if self.Km == absolute(self.Km) {"+"}  else {"-"},
          //                                          absolute(self.Km));
    }
}

impl PartialEq for Quaternion {
    fn eq(&self, other: &Quaternion) -> bool {
        return (self.Re == other.Re)&&(self.Im == other.Im)&&(self.Jm==other.Jm)&&(self.Km==other.Km);
    }
}

impl Quaternion {
    fn zero() -> Self {
        return quaternion!(0,0,0,0);
    }

    fn one() -> Self {
        return quaternion!(1,0,0,0);
    }

    fn newi() -> Self {
        return quaternion!(0,1,0,0);
    }

    fn newj() -> Self {
        return quaternion!(0,0,1,0);
    }

    fn newk() -> Self {
        return quaternion!(0,0,0,1);
    }

    fn real(&self) -> bool {
        return !(self.Im != 0.0 || self.Jm != 0.0 || self.Km != 0.0);
    }

    fn norm(&self) -> f64 {
        return (self.Re.powi(2) + self.Im.powi(2) + self.Jm.powi(2) + self.Km.powi(2)).sqrt();
    }

    fn apply(&self, f: impl Fn(f64) -> f64) -> Self {
        return Self {
            Re: f(self.Re),
            Im: f(self.Im),
            Jm: f(self.Jm),
            Km: f(self.Km)
        };
    }

    fn conjugate(&self) -> Self {
        return ((*self) + Quaternion::newi()*(*self)*Quaternion::newi() +
            Quaternion::newj()*(*self)*Quaternion::newj() +
            Quaternion::newk()*(*self)*Quaternion::newk()).apply(|x|{x*-0.5});
    }

    fn inverse(&self) -> Self {
        return self.conjugate().apply(
            |x| {x/self.norm().powi(2)}
        );
    }

    fn unit_of(&self) -> Self {
        return self.apply(
            |x| {x/self.norm()}
        )
    }

}
//QUATERNIONS END


fn main() {
    let n = quaternion!(-2,-2,-3.324,4);
    println!("{} * {} * {} = {}", Quaternion::newi().apply(|x| {x*-1.0}), Quaternion::newj(), Quaternion::newk(),
        (Quaternion::newi().apply(|x| {x*-1.0}))*Quaternion::newj()*Quaternion::newk());

    println!("{}",n.conjugate() * n)
}
