// rc1.rs
//
// In this exercise, we want to express the concept of multiple owners via the
// Rc<T> type. This is a model of our solar system - there is a Sun type and
// multiple Planets. The Planets take ownership of the sun, indicating that they
// revolve around the sun.
//
// Make this code compile by using the proper Rc primitives to express that the
// sun has multiple owners.
//
// Execute `rustlings hint rc1` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

use std::rc::Rc;
#[derive(Debug)]
struct Sun {}
#[derive(Debug)]
enum Planet {
 Mercury(Rc<Sun>),
 Venus(Rc<Sun>),
 Earth(Rc<Sun>),
 Mars(Rc<Sun>),
 Jupiter(Rc<Sun>),
 Saturn(Rc<Sun>),
 Uranus(Rc<Sun>),
 Neptune(Rc<Sun>),
}
impl Planet {
 fn details(&self) {
 println!("Hi from {:?}!", self)
 }
}
#[test]
fn main() {
 let sun = Rc::new(Sun {});
 println!("reference count = {}", Rc::strong_count(&sun)); 
 let mercury = Planet::Mercury(Rc::clone(&sun));
 println!("reference count = {}", Rc::strong_count(&sun)); 
 mercury.details();
 let venus = Planet::Venus(Rc::clone(&sun));
 println!("reference count = {}", Rc::strong_count(&sun)); 
 venus.details();
 let earth = Planet::Earth(Rc::clone(&sun));
 println!("reference count = {}", Rc::strong_count(&sun)); 
 earth.details();
 let mars = Planet::Mars(Rc::clone(&sun));
 println!("reference count = {}", Rc::strong_count(&sun)); 
 mars.details();
 let jupiter = Planet::Jupiter(Rc::clone(&sun));
 println!("reference count = {}", Rc::strong_count(&sun)); 
 jupiter.details();
 // TODO
 let saturn = Planet::Saturn(Rc::new(Sun {}));
 println!("reference count = {}", Rc::strong_count(&match &saturn { Planet::Saturn(sun) => 
Rc::clone(sun), _ => unreachable!() })); 
 saturn.details();
 // TODO
 let uranus = Planet::Uranus(Rc::new(Sun {}));
 println!("reference count = {}", Rc::strong_count(&match &uranus { Planet::Uranus(sun) => 
Rc::clone(sun), _ => unreachable!() })); 
 uranus.details();
 // TODO
 let neptune = Planet::Neptune(Rc::new(Sun {}));
 println!("reference count = {}", Rc::strong_count(&match &neptune { Planet::Neptune(sun) => 
Rc::clone(sun), _ => unreachable!() })); 
 neptune.details();
 assert_eq!(Rc::strong_count(&sun), 9);
 drop(neptune);
 println!("reference count = {}", Rc::strong_count(&sun)); 
 drop(uranus);
 println!("reference count = {}", Rc::strong_count(&sun)); 
 drop(saturn);
 println!("reference count = {}", Rc::strong_count(&sun)); 
 drop(jupiter);
 println!("reference count = {}", Rc::strong_count(&sun)); 
 drop(mars);
 println!("reference count = {}", Rc::strong_count(&sun)); 
 println!("reference count = {}", Rc::strong_count(&sun)); 
 println!("reference count = {}", Rc::strong_count(&sun)); 
 println!("reference count = {}", Rc::strong_count(&sun)); 
 assert_eq!(Rc::strong_count(&sun), 1);
}
