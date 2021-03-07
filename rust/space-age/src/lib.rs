// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.


// derive a struct duration which will help extract meaningful information
// from what is usually given

#[derive(Debug)]
pub struct Duration
{
    float: f64,
    // minutes: i32,
}

impl From<u64> for Duration {

    // in this case, we know that we will get input in raw seconds
    // but those seconds need to be converted into earth years
    // as the required output is in years

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

    // the default implementation which works for the 8 planets
    // if we had a Special_Planet whose years_during was not as simple
    // that would be implemented in the respective `impl Planet for Special_Planet`
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

// TODO
// hardcode the orbital periods in the respective structs as that should
// be accessible when someone works with the struct

// then Planet's const orbital_period should use that value

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
