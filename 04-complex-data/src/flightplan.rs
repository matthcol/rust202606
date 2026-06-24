// adep
// ades
// fl

use std::{fmt::Display, ops::AddAssign};

#[derive(Debug)]
pub struct FlightPlan{
    pub adep: String,
    pub ades: String,
    pub fl: u16
}

impl FlightPlan {
    pub fn new(adep: &str, ades: &str, fl: u16) -> FlightPlan{
        FlightPlan { 
            adep: String::from(adep), 
            ades: String::from(ades), 
            fl: fl 
        }
    }

    // TODO: other creation methods ?
    // TODO: "object" methods
    // TODO: traits
}

impl From<(&str, &str, u16)> for FlightPlan  {
    fn from(value: (&str, &str, u16)) -> Self {
        FlightPlan::new(value.0, value.1, value.2)
    }
}

impl From<(&str, &str)> for FlightPlan {
    fn from(value: (&str, &str)) -> Self {
        FlightPlan::new(value.0, value.1, 100)
    }
}

// NB: already there when implementing From
// impl Into<FlightPlan> for (&str, &str, u16){
//     fn into(self) -> FlightPlan {
//         FlightPlan::new(self.0,self.1, self.2)
//         // FlightPlan::from(self)
//     }
// }

impl Display for FlightPlan{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {} ({})", self.adep, self.ades, self.fl)
    }
}

// NB: homogeneous implementation => Rhs = FlightPlan
// impl AddAssign for FlightPlan{
//     fn add_assign(&mut self, rhs: Self) {
//         todo!()
//     }
// }
impl AddAssign<u16> for FlightPlan {
    fn add_assign(&mut self, rhs: u16) {
        self.fl += rhs
    }
}