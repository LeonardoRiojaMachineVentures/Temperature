enum Temperature {
	Celcius(f64),
	Kelvin(f64),
	Fahrenheit(f64),
}
enum TempClass {
	Celcius,
	Kelvin,
	Fahrenheit,
}
impl Temperature {
	fn to_celcius(self) -> Temperature {
		match self {
			Temperature::Fahrenheit(*v) => Temperature::Celcius(5.0f64*(v - 32.0f64)/9.0f64),
			Temperature::Kelvin(*v) => Temperature::Celcius(v - 273.15f64),
			Temperature::Celcius(*v) => Temperature::Celcius(v),
		}
	}
	fn to_kelvin(self) -> Temperature {
		match self {
			Temperature::Fahrenheit(*v) => Temperature::Kelvin(273.15f64 + 5.0f64*(v - 32.0f64)/9.0f64),
			Temperature::Celcius(*v) => Temperature::Kelvin(v + 273.15f64),
			Temperature::Kelvin(*v) => Temperature::Kelvin(v),
		}
	}
	fn to_fahrenheit(&mut self) -> Temperature {
		match self {
			Temperature::Fahrenheit(*v) => Temperature::Fahrenheit(v),
			Temperature::Kelvin(*v) => Temperature::Fahrenheit(32.0f64 + 9.0f64*(v - 273.15f64)/5.0f64),
			Temperature::Celcius(*v) => Temperature::Fahrenheit(32.0f64 + 9.0f64*(v)/5.0f64),
		}
	}
	fn new(t : TempClass, x : f64) -> Result<Temperature, ()> {
		match t {
			TempClass::Celcius => {
				if x >= -273.15f64 {Ok(Temperature::Celcius(x))}
				else {Err(())}
			},
			TempClass::Kelvin => {
				if x  >= 0.0f64 {Ok(Temperature::Kelvin(x))}
				else {Err(())}
			},
			TempClass::Fahrenheit => {
				if x >= -459.67f64 {Ok(Temperature::Fahrenheit(x))}
				else {Err(())}
			},
		}
	}
}

fn main() -> () {	
	let low_temp = Temperature::new(TempClass::Fahrenheit, 200.0f64).unwrap();
	let mid_temp = Temperature::new(TempClass::Fahrenheit, 300.0f64).unwrap();
	let high_temp = Temperature::new(TempClass::Fahrenheit, 400.0f64).unwrap();
	let high_temp_increased = Temperature::new(TempClass::Fahrenheit, 500.0f64).unwrap();
	let extreme_temp = Temperature::new(TempClass::Fahrenheit, 600.0f64).unwrap();

	low_temp.to_kelvin();
	mid_temp.to_kelvin();
	high_temp.to_kelvin();
	high_temp_increased.to_kelvin();
	extreme_temp.to_kelvin();
	

}


