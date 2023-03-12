#![allow(dead_code,non_snake_case)]
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use core::ops::Add;
use core::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::fmt::Debug;
use std::ops::AddAssign;


pub fn absolute(x: f64) -> f64 {
    return if x >= 0.0 {x} else {-x};
}

pub fn max(x: f64, y: f64) -> usize {
    return if x >= y {x as usize} else {y as usize};
}

pub fn min(x: f64, y: f64) -> usize {
    return if x <= y {x as usize} else {y as usize};
}


//SOME TRAITS WE'RE GONNA NEED
pub trait Vectorable {
    fn vectorify(&self) -> Vec<f64>;
}

pub trait Zero {
    fn zero() -> Self;
}

pub trait One {
    fn one() -> Self;
}
//TRAITS END


//SOME TRAIT IMPLEMENTATIONS (First for Zero trait and then for One trait)
impl Zero for f64 {
    fn zero() -> f64 {
        0.0
    }
}
impl Zero for f32 {
    fn zero() -> f32 {
        0.0
    }
}
impl Zero for i32 {
    fn zero() -> i32 {
        0.0 as i32
    }
}
impl Zero for i64 {
    fn zero() -> i64 {
        0.0 as i64
    }
}
impl Zero for u32 {
    fn zero() -> u32 {
        0.0 as u32
    }
}
impl Zero for u64 {
    fn zero() -> u64 {
        0.0 as u64
    }
}
impl Zero for usize {
    fn zero() -> usize {
        0.0 as usize
    }
}

impl One for f64 {
    fn one() -> f64 {
        1.0
    }
}
impl One for f32 {
    fn one() -> f32 {
        1.0
    }
}
impl One for i32 {
    fn one() -> i32 {
        1.0 as i32
    }
}
impl One for i64 {
    fn one() -> i64 {
        1.0 as i64
    }
}
impl One for u32 {
    fn one() -> u32 {
        1.0 as u32
    }
}
impl One for u64 {
    fn one() -> u64 {
        1.0 as u64
    }
}
impl One for usize {
    fn one() -> usize {
        1.0 as usize
    }
}
//TRAIT IMPLEMENTATION END


//COMPLEX START
pub struct ComplexNumber {
    pub Re: f64,
    pub Im: f64
}

macro_rules! complex {
    ($ex: expr, $ex2: expr) => {
        ComplexNumber {
            Re: $ex as f64,
            Im: $ex2 as f64
        }
    }
}

impl PartialEq for ComplexNumber {
    fn eq(&self, other: &ComplexNumber) -> bool {
        return (self.Re == other.Re)&&(self.Im == other.Im);
    }
}

impl Copy for ComplexNumber {
}

impl Clone for ComplexNumber {
    fn clone(&self) -> Self {
        Self {
            Re: self.Re,
            Im: self.Im
        }
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

impl Add<ComplexNumber> for f64 {
    type Output = ComplexNumber;
    fn add(self, other: ComplexNumber) -> ComplexNumber {
        ComplexNumber {
            Re: self + other.Re,
            Im: other.Im,
        }
    }
}

impl Add<f64> for ComplexNumber {
    type Output = ComplexNumber;
    fn add(self, other: f64) -> ComplexNumber {
        ComplexNumber {
            Re: other + self.Re,
            Im: self.Im,
        }
    }
}

impl AddAssign for ComplexNumber {
    fn add_assign(&mut self, other: ComplexNumber) -> () {
        *self = ComplexNumber {
            Re: self.Re + other.Re,
            Im: self.Im + other.Im
        };
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

impl Div<ComplexNumber> for ComplexNumber {
    type Output = ComplexNumber;
    fn div(self, other: ComplexNumber) -> ComplexNumber {
        let mut temp: Self = (self * other.conjugate()).apply(|x: f64| {x/(other.Re.powi(2) + other.Im.powi(2))});
        return Self {
            Re: temp.Re,
            Im: temp.Im
        };
    }
}

impl Default for ComplexNumber {
    fn default() -> Self {
        complex!(0,1)
    }
} 

impl Zero for ComplexNumber {
    fn zero() -> Self {
        complex!(0,0)
    }
}

impl Display for ComplexNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut text = format!("");
        if self.Re != 0.0 || self.Im == 0.0 {
            text = format!("{}",self.Re);
        }
        if self.Im != 0.0 {
            text = format!("{}{}{}i",text,
                if self.Re != 0.0 {if self.Im > 0.0{"+"}else {"-"}}
                else {if self.Im > 0.0{""}else {"-"}} ,
                if absolute(self.Im) != 1.0 {absolute(self.Im).to_string()}
                else {"".to_string()});
        }
        write!(f, "{}", text)
    }
}

impl Debug for ComplexNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut text = format!("");
        if self.Re != 0.0 || self.Im == 0.0 {
            text = format!("{}",self.Re);
        }
        if self.Im != 0.0 {
            text = format!("{}{}{}i",text,
                if self.Re != 0.0 {if self.Im > 0.0{"+"}else {"-"}}
                else {if self.Im > 0.0{""}else {"-"}} ,
                if absolute(self.Im) != 1.0 {absolute(self.Im).to_string()}
                else {"".to_string()});
        }
        write!(f, "{}", text)
    }
}

impl Vectorable for ComplexNumber {
    fn vectorify(&self) -> Vec<f64> {
        return vec![self.Re, self.Im ];
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
//COMPLEX END

//QUATERNIONS START
pub struct Quaternion {
    pub Re: f64,
    pub Im: f64,
    pub Jm: f64,
    pub Km: f64,
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

impl Add<f64> for Quaternion {
    type Output = Quaternion;
    fn add(self, other:f64) -> Quaternion {
        return Quaternion {
            Re: self.Re + other,
            Im: self.Im,
            Jm: self.Jm,
            Km: self.Km,
        }
    }
}

impl Add<Quaternion> for f64 {
    type Output = Quaternion;
    fn add(self, other:Quaternion) -> Quaternion {
        return Quaternion {
            Re: other.Re + self,
            Im: other.Im,
            Jm: other.Jm,
            Km: other.Km,
        }
    }
}

impl AddAssign for Quaternion {

    fn add_assign(&mut self, other: Quaternion) -> () {
        *self = Quaternion {
            Re: self.Re + other.Re,
            Im: self.Im + other.Im,
            Jm: self.Jm + other.Jm,
            Km: self.Km + other.Km,
        };
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

impl Default for Quaternion {
    fn default() -> Self {
        quaternion!(0,1,1,1)
    }
}

impl Zero for Quaternion {
    fn zero() -> Self {
        quaternion!(0,0,0,0)
    }
}

impl Display for Quaternion {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut text: String = format!("");
        if self.Re != 0.0 || (self.Im == 0.0 && self.Jm == 0.0 && self.Km == 0.0) {
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
    }
}


impl Debug for Quaternion {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut text: String = format!("");
        if self.Re != 0.0 || (self.Im == 0.0 && self.Jm == 0.0 && self.Km == 0.0) {
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
    }
}

impl PartialEq for Quaternion {
    fn eq(&self, other: &Quaternion) -> bool {
        return (self.Re == other.Re)&&(self.Im == other.Im)&&(self.Jm==other.Jm)&&(self.Km==other.Km);
    }
}

impl Vectorable for Quaternion {
    fn vectorify(&self) -> Vec<f64> {
        return vec![self.Re, self.Im, self.Jm, self.Km];
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

//MATRIX START
pub struct Matrix {
    pub height: usize,
    pub width: usize,
    pub data: Vec<Vec<f64>>
}

macro_rules! matrix {

    ($ex:expr, $ex2:expr) => {
        Matrix {
            height: $ex,
            width: $ex2,
            data: vec![vec![0f64;$ex2];$ex]
        }
    };

    ($ex:expr, $ex2:expr, $ex3:expr) => {
        Matrix :: new($ex, $ex2, $ex3)
    };

}

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut temp: String = "".to_string();
        for x in 0..self.height {
            temp = format!("{}{:?}\n" ,temp,self.data[x]);
        }
        return write!(f,"{}", temp);
    }
}

impl Add<Matrix> for Matrix {
    type Output = Matrix;
    fn add(self, other: Matrix) -> Matrix {
        let mut temp = matrix!(self.height, self.width);
        for x in 0..self.height {
            for y in 0..self.width {
                temp.data[x][y] = self.data[x][y] + other.data[x][y];
            }
        }
        return temp;
    }
} 

impl Sub<Matrix> for Matrix {
    type Output = Matrix;
    fn sub(self, other: Matrix) -> Matrix {
        let mut temp = matrix!(self.height, self.width);
        for x in 0..self.height {
            for y in 0..self.width {
                temp.data[x][y] = self.data[x][y] - other.data[x][y];
            }
        }
        return temp;
    }
} 

impl Mul<Matrix> for Matrix {
    type Output = Matrix;
    fn mul(self, other: Matrix) -> Matrix {
        let mut temp = matrix!(self.height, self.width);
        for x in 0..self.height {
            for y in 0..self.width {
                for k in 0..self.width {
                    temp.data[x][y] += self.data[x][k] * other.data[k][y];
                }
            }
        }
        return temp;
    }
} 

impl Mul<f64> for Matrix {
    type Output = Matrix;
    fn mul(self, other: f64) -> Matrix {
        return self.apply(|x| {x*other});
    }
} 

impl Mul<Matrix> for f64 {
    type Output = Matrix;
    fn mul(self, other:Matrix) -> Matrix {
        return other.apply(|x| {x*self});
    }
}

impl Div<f64> for Matrix {
    type Output = Matrix;
    fn div(self, other: f64) -> Matrix {
        return self.apply(|x| {x/other});
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Matrix) -> bool {
        if self.height != other.height || self.width != other.width {
            return false;
        }
        for x in 0..self.height {
            for y in 0..self.width {
                if self.data[x as usize][y as usize] != other.data[x as usize][y as usize] {
                    return false;
                }
            }
        }
        return true;
    }
}

impl Matrix {
    fn new(height: usize, width: usize, this: Vec<Vec<f64>>) -> Self {
        if this.len() > height || this[0].len() > width {
            return matrix!(height, width);
        }
        
        let mut temp: Vec<Vec<f64>> = this;
        while temp.len() < height{
            temp.push(vec![0.0;width]);
        }
        
        Self {
            height: height,
            width: width,
            data: temp
        }
    }

    fn identitymat(height: usize) -> Self {
        let mut temp: Self = matrix!(height, height);
        for x in 0..height {
            for y in 0..height {
                if x == y {
                    temp.data[x as usize][y as usize] = 1.0;
                }
            }
        }
        return temp;
    }

    fn minor(&self, i: usize, j: usize) -> Self {
        let mut minor: Self = matrix!(self.height-1,
                                        self.width-1);
        let mut it: usize = 0;
        let mut jt: usize = 0;
        for x in 0..self.height {
            jt = 0;
            for y in 0..self.width {
                if x != i && y != j {
                    minor.data[if it > i.into() {it-1} else{it}]
                         [if jt > j.into() {jt-1} else{jt}] =
                         self.data[x as usize][y as usize];
                }
                jt += 1;
            }
            it += 1;
        }
        return minor;
    }

    fn det(&self) -> f64 {
        if self.height == 1 {return self.data[0][0];}
        if self.height == 2 {
            return self.data[0][0]*self.data[1][1] -
                    self.data[1][0]*self.data[0][1];
        }
        let mut sum: f64 = 0.0;
        for x in 0..self.width {
            if self.data[0][x as usize] != 0.0 {
                sum += self.data[0][x as usize] *
                    (if (x + 2) % 2 == 0 {1.0} else {-1.0}) *
                    self.minor(0,x).det();
            }
        }
        return sum;
    }

    fn trans(&self) -> Self {
       let mut trans: Self = matrix!(self.width,self.height);
        for x in 0..self.height {
            for y in 0..self.width {
                trans.data[y as usize][x as usize] =
                    self.data[x as usize][y as usize];
            }
        }
        return trans;
    }

    fn inverse(&self) -> Option<Self>{
        //returns the inverse multiplied by the determinant of the original
        let mut star: Self = self.trans();
        let det_m = self.det();
        if det_m != 0 as f64 {
            for x in 0..self.height {
                for y in 0..self.width {
                    star.data[x as usize][y as usize] =
                        (if (x+y+2)%2 == 0 {1.0} else {-1.0}) *
                        self.trans().minor(x,y).det() / det_m;
                }
            }
            return Some(star);
        }
        return None;
    }

    fn apply(&self, f: impl Fn(f64) -> f64) -> Self {
        let mut temp = matrix!(self.height, self.width);
        for x in 0..temp.width {
            for y in 0..temp.height {
                temp.data[x as usize][y as usize] = f(self.data[x as usize][y as usize]);
            }
        }
        return temp;
    }

    fn correct(&mut self) {
        for x in 0..self.height {
            for y in 0..self.width {
                let elem = self.data[x as usize][y as usize];
                let mut flem = self.data[x as usize][y as usize].floor();
                if elem - flem > 0.999999 {
                    self.data[x as usize][y as usize] =  elem.round();
                }
                if  elem > 0.0 && elem < 0.00001 {
                    self.data[x as usize][y as usize] =  elem.round();
                }
                if  elem  < 0.0 &&  elem  > -0.00001 {
                    self.data[x as usize][y as usize] =  elem.round();
                }
            }
        }
    }

    fn replace_collumn_with(&self, collumn: usize, new_collumn: Vec<f64>) -> Self {
        let mut temp = self.data.clone();
        for row in 0..min(temp.len() as f64, new_collumn.len() as f64) {
            temp[row as usize][collumn as usize] = new_collumn[row as usize];
        }
        return Self {
            height: self.height,
            width: self.width,
            data: temp
        };
    }

    fn replace_line_with(&self, line: usize, new_line: Vec<f64>) -> Self {
        let mut temp = self.data.clone();
        for col in 0..min(temp[0].len() as f64, new_line.len() as f64) {
            temp[line as usize][col as usize] = new_line[col as usize];
        }
        return Self {
            height: self.height,
            width: self.width,
            data: temp
        };
    }

    fn clone(&self) -> Self {
        return matrix!(self.height, self.width, self.data.clone());
    }

}
//MATRIX END

//GOOD MATRIX START

pub struct Matrice<T> {
    pub height: usize,
    pub width: usize,
    pub data: Vec<Vec<T>>
}

/*macro_rules!  matrice {
    ($ex1:expr, $ex2:expr) => {
        Matrice<f64> {
            height: $ex1,
            width: $ex2,
            data: vec![vec![0.0;$ex1];$ex2];
        }
    };

    ($t:ty, $ex1:expr, $ex2:expr) => {
        Matrice::<$t> {
            height: $ex1,
            width: $ex2,
            data: vec![vec![($t)::default();$ex1];$ex2]
        }
    }
}*/

impl<T: Display + Default + std::clone::Clone + Copy + Debug + Add + Add<Output = T> + Mul + Mul<Output = T> + Zero + PartialEq + Sub<Output = T>> Add<Matrice<T>> for Matrice<T> {
    type Output = Matrice<T>;
    fn add(self, other: Matrice<T>) -> Matrice<T> {
        let mut temp = Matrice::<T>::new(self.height, self.width);
        for x in 0..temp.height {
            for y in 0..temp.width {
                temp.data[x as usize][y as usize] = 
                    self.data[x as usize][y as usize] +
                    other.data[x as usize][y as usize];
            }
        }
        return temp;
    } 
}

impl<T: Display + Default + std::clone::Clone + Copy + Debug + Add + Add<Output = T> + Mul + Mul<Output = T> + Zero + PartialEq + Sub<Output = T>> Display for Matrice<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut text = format!("{:?}", self.data[0usize]);
        for x in 1..self.height {
            text = format!("{}\n{:?}", text, self.data[x as usize]);
        }
        write!(f,"{}", text)
    }
}

impl<T: Display + Default + std::clone::Clone + Copy + Debug + Add + Add<Output = T> + Mul + Mul<Output = T> + Zero + PartialEq + Sub<Output = T>>  Matrice<T> {
    fn new(height:usize, width:usize) -> Self {
        Self {
            height: height,
            width: width,
            data: vec![vec![T::default();width];height]
        }
    }

    fn new_zero(height:usize, width:usize) -> Self {
        Self {
            height: height,
            width: width,
            data: vec![vec![T::zero();width];height]
        }
    }

    fn minor(&self, i: usize, j: usize) -> Self {
        let mut minor: Self = Self::new_zero(self.height-1,
                                        self.width-1);
        let mut it: usize = 0;
        let mut jt: usize = 0;
        for x in 0..self.height {
            jt = 0;
            for y in 0..self.width {
                if x != i && y != j {
                    minor.data[if it > i.into() {it-1} else{it}]
                         [if jt > j.into() {jt-1} else{jt}] =
                         self.data[x as usize][y as usize];
                }
                jt += 1;
            }
            it += 1;
        }
        return minor;
    }

    fn det(&self) -> T {
        todo!()
    }

    fn printm(&self) -> () {
        self.data.iter().for_each(|v|{println!("{:?}",v)});
    }
}

//GOOD MATRIX END


fn main() {
    let n = quaternion!(-2,-2,-3.324,4);
    println!(" Quaternions demonstration: {} * {} * {} = {}", Quaternion::newi().apply(|x| {x*-1.0}), Quaternion::newj(), Quaternion::newk(),
        (Quaternion::newi().apply(|x| {x*-1.0}))*Quaternion::newj()*Quaternion::newk());
    println!(" Complex numbers demonstration: {} * {} = {}", ComplexNumber::i().apply(|x| {x*-1.0}), ComplexNumber::i(),
        (ComplexNumber::i().apply(|x| {x*-1.0}))*ComplexNumber::i());

    let mut mat: Matrice<ComplexNumber>  = Matrice::<ComplexNumber>::new(8, 8);
    println!(" Default <ComplexNumber> Matrix of size 4x4 :\n{}\n", Matrice::<ComplexNumber>::new(4, 4));
    println!(" Default <Quaternion> Matrix of size 4x4 :\n{}\n", Matrice::<Quaternion>::new(4, 4));
    println!(" Default <f64> Matrix of size 4x4 :\n{}\n", Matrice::<f64>::new(4, 4));
    println!(" Default <i32> Matrix of size 4x4 :\n{}\n", Matrice::<i32>::new(4, 4));
    //let mut temp = matrix!(mat.height, mat.width, mat.data.clone());
    //if let Some(a) = temp.clone().inverse() {
    //    println!("{}", a*temp);
    //}

    println!("{} / {} = {}", complex!(12, 13), complex!(0, 3), (complex!(12, 13)/complex!(0, 3)));

    println!("{}", quaternion!(0,0,0,0));
    //mat.trans().data.iter().for_each(|v|{println!("{:?}", v)});
 }
