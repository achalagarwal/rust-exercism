// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.


#[derive(Debug)]
pub struct Duration
{
    float: f64,
    // minutes: i32,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        // return
         Duration{
             float: s as f64/31557600.0 as f64
         }
        // s as f64
        // Duration()
        // unimplemented!("s, measured in seconds: {}", s)
    }
}

pub trait Planet {
    const ORBITAL_PERIOD: f64;
    fn years_during(d: &Duration) -> f64 {

        d.float/Self::ORBITAL_PERIOD
        
        // unimplemented!(
        //     "convert a duration ({:?}) to the number of years on this planet for that duration",
        //     d,
        // );
    }
}
 
pub struct Mercury{
    // orbital_period: op_mercury
    // orbital_period: f64
}
pub struct Venus{
    // orbital_period: op_venus
}
pub struct Earth{
    // orbital_period: op_earth
}
pub struct Mars{
    // orbital_period: op_mars
}
pub struct Jupiter{
    // orbital_period: op_jupiter
}
pub struct Saturn{
    // orbital_period: op_saturn
}
pub struct Uranus{
    // orbital_period: f64
}
pub struct Neptune{
    // orbital_period: op_neptune
}

impl Planet for Mercury {
    const ORBITAL_PERIOD: f64 = 0.2408467;
    // fn years_during(&self, d: &Duration) -> f64 {
    //     d.float*self.orbital_period
    // }
}
impl Planet for Venus {
    const ORBITAL_PERIOD: f64 = 0.61519726;
}
impl Planet for Earth {
    const ORBITAL_PERIOD: f64 = 1.0;
}
impl Planet for Mars {
    const ORBITAL_PERIOD: f64 = 1.8808158;
}
impl Planet for Jupiter {
    const ORBITAL_PERIOD: f64 = 11.862615;
}
impl Planet for Saturn {
    const ORBITAL_PERIOD: f64 = 29.447498;
}
impl Planet for Uranus {
    const ORBITAL_PERIOD: f64 = 84.016846;
}
impl Planet for Neptune {
    const ORBITAL_PERIOD: f64 = 164.79132;
}
