
#[derive(Debug, Clone, Default)]
struct NumberWithUnit{
    unit: String,
    value: f64,
}

impl NumberWithUnit {

    pub fn unitleless(value: f64) -> Self {
        NumberWithUnit {
            value,
            unit: String::new(),
        }
    }

    pub fn with_unit(value: f64, unit: String) -> Self{
        NumberWithUnit { value, unit }
    }

    pub fn with_unit_from(other: Self, value: f64) -> Self {
        NumberWithUnit {
            value,
            unit: other.unit,
        }
    }

    pub fn add(self, other: Self) -> Self {
        if self.unit != other.unit {
            panic!("Cannot add numbers with different units");
        }
        NumberWithUnit {
            value: self.value + other.value,
            unit: self.unit,
        }
    }

    pub fn mul(self, other: Self) -> Self {
        let new_unit = if self.unit.is_empty() {
            other.unit
        } else if other.unit.is_empty() {
            self.unit
        } else {
            format!("{}*{}", self.unit, other.unit)
        };
        NumberWithUnit {
            value: self.value * other.value,
            unit: new_unit,
        }
    }

    pub fn div(self, other: Self) -> Self {
        let new_unit = if self.unit.is_empty() {
            format!("1/{}", other.unit)
        } else if other.unit.is_empty() {
            self.unit
        } else {
            format!("{}/{}", self.unit, other.unit)
        };
        NumberWithUnit {
            value: self.value / other.value,
            unit: new_unit,
        }
    }

    pub fn add_in_place(&mut self, other: &Self) {
        if self.unit != other.unit {
            panic!("Cannot add numbers with different units");
        }
        self.value += other.value;
    }

    pub fn mul_in_place(&mut self, other: &Self) {
        self.value *= other.value;
        if self.unit.is_empty()
        {
            self.unit = other.unit.clone();
        }
        else if !other.unit.is_empty()
        {
            self.unit = format!("{}*{}", self.unit, other.unit)
        }
    }

    pub fn div_in_place(&mut self, other: &Self) {
        self.value /= other.value;
        if self.unit.is_empty() {
            self.unit = format!("1/{}", other.unit);
        } else if !other.unit.is_empty() {
            self.unit = format!("{}/{}", self.unit, other.unit)
        }
    }

    pub fn mul_vals(value: &[NumberWithUnit]) -> Self {
        let mut result = NumberWithUnit::unitleless(1.0);
        for num in value {
            result.mul_in_place(num);
        }
        result
    }

    
    pub fn mul_vals_vec(value: Vec<NumberWithUnit>) -> Self {
        let mut result = NumberWithUnit::unitleless(1.0);
        for num in value {
            result = result.mul(num);
        }
        result
    }

}

struct DoubleString (String, String);

impl DoubleString {

    fn from_strs(str_1: &str, str_2: &str) -> Self {
        DoubleString(str_1.to_string(), str_2.to_string())
    }

    fn from_strings(str_1: &String, str_2: &String) -> Self {
        DoubleString(str_1.clone(), str_2.clone())
    }

    fn show(&self) {
        println!("({}, {})", self.0, self.1);
    }
}

fn main() {
    let unitleless_num = NumberWithUnit::unitleless(3.0);
    println!("Unitleless: {:?}", unitleless_num);

    let speed = NumberWithUnit::with_unit(9.81, String::from("m/s^2"));
    println!("Speed: {:?}", speed);

    let default_num = NumberWithUnit::default();
    println!("Default: {:?}", default_num);

    let speed_clone = speed.clone();
    println!("Speed Clone: {:?}", speed_clone);

    let speed_for_distance = speed.clone();
    let extra_speed = NumberWithUnit::with_unit_from(speed_for_distance, 100.0);
    println!("Extra speed: {:?}", extra_speed);

    println!("-------New Operations-------");

    // add

    let first_distance = NumberWithUnit::with_unit(50.0, String::from("m"));
    println!("First distance: {:?}", first_distance);
    let second_distance = NumberWithUnit::with_unit(70.0, String::from("m"));
    println!("Second distance: {:?}", second_distance);
    let total_distance = first_distance.add(second_distance);
    println!("Total distance: {:?}", total_distance);

    // div

    let time = NumberWithUnit::with_unit(10.0, String::from("s"));
    println!("Time: {:?}", time);
    let new_speed = total_distance.div(time);
    println!("Speed: {:?}", speed);

    // mul

    let new_time = NumberWithUnit::with_unit(2.0, String::from("s"));
    let multiply = new_speed.mul(new_time);
    println!("Multiply: {:?}", multiply);

    println!("-------In-place Operations-------");

    // add
    let mut in_place_distance = NumberWithUnit::with_unit(50.0, String::from("m"));
    println!("In-place first distance before: {:?}", in_place_distance);
    let in_place_second_distance = NumberWithUnit::with_unit(70.0, String::from("m"));
    println!("In-lace second distance before: {:?}", in_place_second_distance);
    in_place_distance.add_in_place(&in_place_second_distance);
    println!("In-place total distance after: {:?}", in_place_distance);

    // div
    let time = NumberWithUnit::with_unit(10.0, String::from("s"));
    println!("In-place time before: {:?}", time);
    in_place_distance.div_in_place(&time);
    println!("In-place speed after: {:?}", in_place_distance);

    // mul
    let new_time = NumberWithUnit::with_unit(2.0, String::from("s"));
    println!("In-place new time before: {:?}", new_time);
    in_place_distance.mul_in_place(&new_time);
    println!("In-place multiply after: {:?}", in_place_distance);

    println!("-------Multiplying Slice-------");
    let values = vec![
        NumberWithUnit::with_unit(2.0, String::from("m")),
        NumberWithUnit::with_unit(3.0, String::from("m")),
        NumberWithUnit::with_unit(4.0, String::from("m")),
    ];
    let multiplied = NumberWithUnit::mul_vals(&values);
    println!("Multiplied slice: {:?}", multiplied);
    let multiplied_2 = NumberWithUnit::mul_vals(&values);
    println!("Multiplied slice: {:?}", multiplied_2);

    let pvec1 = NumberWithUnit::mul_vals_vec(values.clone());
    let pvec2 = NumberWithUnit::mul_vals_vec(values.clone());
    println!("pvec1 = {:?}, pvec2 = {:?}", pvec1, pvec2);

    // Tuple

    println!("-------Tuples-------");

    let string: String = String::from("hello");
    let str_slice: &str = "world";

    println!("string: '{}', str_slice: '{}'", string, str_slice);

    let ds1 = DoubleString::from_strs(&string, str_slice);
    
    // let ds2 = DoubleString::from_strings(&string, str_slice);
    let str_slice_as_string = str_slice.to_string();
    let ds2 = DoubleString::from_strings(&string, &str_slice_as_string);
    ds1.show();
    ds2.show();
}


