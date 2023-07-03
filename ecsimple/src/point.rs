
use crate::bngf2m::*;



#[derive(Clone)]
pub struct ECGf2mPoint {
	x :BnGf2m,
	y :BnGf2m,
	z :BnGf2m,
	curvename :String,
}

impl std::fmt::Display for ECGf2mPoint {
	fn fmt(&self, f:&mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f,"curve[{}] x 0x{:x} y 0x{:x} z 0x{:x}", self.curvename,self.x,self.y,self.z)
	}
}

impl std::default::Default for ECGf2mPoint {
	fn default() -> Self {
		ECGf2mPoint {
			x : BnGf2m::default(),
			y :BnGf2m::default(),
			z :BnGf2m::default(),
			curvename : "".to_string(),
		}
	}
}


impl ECGf2mPoint {
	pub fn new(name :&str) -> ECGf2mPoint {
		ECGf2mPoint {
			x : BnGf2m::default(),
			y :BnGf2m::default(),
			z :BnGf2m::default(),
			curvename : format!("{}",name),
		}
	}

	pub fn set_x(&mut self, x :&BnGf2m) {
		self.x = x.clone();
	}

	pub fn set_y(&mut self, y :&BnGf2m) {
		self.y = y.clone();
	}

	pub fn set_z(&mut self, z :&BnGf2m) {
		self.z = z.clone();
	}

}