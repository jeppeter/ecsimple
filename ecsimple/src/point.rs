
use crate::bngf2m::*;
use crate::group::*;


#[derive(Clone)]
pub struct ECGf2mPoint {
	x :BnGf2m,
	y :BnGf2m,
	z :BnGf2m,
	group :ECGroupBnGf2m,
}

impl std::fmt::Display for ECGf2mPoint {
	fn fmt(&self, f:&mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f,"curve[{}] x 0x{:x} y 0x{:x} z 0x{:x}", self.group,self.x,self.y,self.z)
	}
}

impl std::default::Default for ECGf2mPoint {
	fn default() -> Self {
		ECGf2mPoint {
			x : BnGf2m::default(),
			y :BnGf2m::default(),
			z :BnGf2m::default(),
			group : ECGroupBnGf2m::default(),
		}
	}
}


impl ECGf2mPoint {
	pub fn new(grp :&ECGroupBnGf2m) -> ECGf2mPoint {
		ECGf2mPoint {
			x : BnGf2m::default(),
			y :BnGf2m::default(),
			z :BnGf2m::default(),
			group : grp.clone(),
		}
	}

	pub fn new_point(x :&BnGf2m, y :&BnGf2m,z :&BnGf2m, grp :&ECGroupBnGf2m) -> Self {
		Self {
			x :x.clone(),
			y :y.clone(),
			z :z.clone(),
			group :grp.clone(),
		}
	}

	pub fn set_group(&mut self, grp :&ECGroupBnGf2m) {
		self.group = grp.clone();
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