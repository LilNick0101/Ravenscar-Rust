use heapless::Vec;
//use cortex_m_semihosting::hprintln;
use libm::{cosf, exp, fabs, log, sinf, sqrt};
use production_workload_constants::*;

type WhetFloat = f64;

pub mod production_workload_constants{
    use super::WhetFloat;

    pub const T : WhetFloat = 0.499975;
    pub const T1 : WhetFloat = 0.50025;
    //pub const T2 : WhetFloat = 2.0;

    pub const N8 : i32 = 10;
    pub const N9 : i32 = 7;

    pub const VALUE : WhetFloat = 0.941377;
    pub const TOLERANCE : WhetFloat = 0.00001;
    //static mut i : i32 = 0;

    pub const Y : WhetFloat = 1.0;
}

pub struct ProductionWorkload{
    e1 : Vec<WhetFloat,7>,
    ij : i32,
    ik : i32,
    il : i32,
    i : i32,
    z : WhetFloat,
    sum : WhetFloat,
}

impl ProductionWorkload{

    pub fn new() -> Self {
        ProductionWorkload{
            e1 : Vec::from_slice(&[0.0;N9 as usize]).unwrap(),
            ij : 0,
            ik : 0,
            il : 0,
            i : 0,
            z : 0.0,
            sum : 0.0,
        }
    }
    
    pub fn small_whetstone(&mut self,kilo_whets : u32){
        for _ in 0..kilo_whets{
            self.clean_array();

            self.ij = (self.ik - self.ij) * (self.il - self.ik);
            self.ik = self.il - (self.ik - self.ij);
            self.il = (self.il - self.ik) * (self.ik + self.il);

            if self.ik - 1 < 0 || self.il - 1 < 0 {
                //println!("Parameter error 3 on small_whetstone");
            }
            else if self.ik - 1 > N9 - 1 || self.il - 1 > N9 - 1 {
                //println!("Parameter error 4 on small_whetstone");
            }
            else{
                self.e1[(self.il - 1) as usize] = (self.ij + self.ik + self.il) as WhetFloat;
                self.e1[(self.ik - 1) as usize] = sinf(self.il as f32) as WhetFloat;
            }

            self.z = self.e1[3];

            for inner_loop in 1..N8 {
                self.z = self.p3(Y * (inner_loop as WhetFloat), Y + self.z);
            }

            self.ij = self.il - (self.il - 3) * self.ik;
            self.il = (self.il - self.ik) * (self.ik - self.ij);
            self.ik = (self.il - self.ik) * self.ik;
            if self.il - 1 < 0 {
                //println!("Parameter error 5 on small_whetstone");
            }
            else if self.il - 1 > N9 - 1 {
                //println!("Parameter error 6 on small_whetstone");
            }
            else {
                self.e1[(self.il - 1) as usize] = (self.ij + self.ik + self.il) as WhetFloat;
            }

            if self.ik + 1 > N9 - 1{
                //println!("Parameter error 7 on small_whetstone");
            } else if self.ik + 1 < 0 {
                //println!("Parameter error 8 on small_whetstone");
            } else {
                self.e1[(self.ik + 1) as usize] = fabs(cosf(self.z as f32) as f64) as WhetFloat;
            }
            
            for _i in 0..N9 {
                self.p0();
            }

            self.z = sqrt(exp(log(self.e1[(N9-1) as usize]) / T1));

            self.sum += self.z;

            if fabs(self.z - VALUE) > TOLERANCE {
                self.sum *= 2.0;
                self.ij += 1;
            }
        }

    }

    fn clean_array(&mut self) {
        for ind in self.e1.iter_mut() {
            *ind = 0.0;
        }
    }

    fn p0(&mut self){
        if self.ij < 0 || self.ik < 0 || self.il < 0 {
            // hprintln!("Parameter error 1 on small_whetstone (P0)").unwrap();
            self.ij = 0;
            self.ik = 0;
            self.il = 0;
        } else if self.ij > production_workload_constants::N9 - 1 || self.ik > production_workload_constants::N9 - 1 || self.il > production_workload_constants::N9 - 1 {
            // hprintln!("Parameter error 2 on small_whetstone (P0)").unwrap();
            self.ij = production_workload_constants::N9 - 1;
            self.ik = production_workload_constants::N9 - 1;
            self.il = production_workload_constants::N9 - 1;
        }
        self.e1[self.ij as usize] = self.e1[self.ik as usize];
        self.e1[self.ik as usize] = self.e1[self.il as usize];
        self.e1[self.i as usize] = self.e1[self.ij as usize];
    }

    fn p3(&mut self, x1 : WhetFloat, y1 : WhetFloat) -> WhetFloat {
        let xtemp = T * (self.z + x1);
        let ytemp = T * (xtemp + y1);

        return ((xtemp + ytemp) / 2.0) as WhetFloat;
    }
}