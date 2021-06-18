struct Car {
    wheel_qty: i16,
    wheel_type: WheelType
}

enum WheelType {
    Big(i16),
    Small(i8)
}

fn main() {
    let a_var = Car{
        wheel_qty: 4,
        wheel_type: WheelType::Small(24)
    };

    // let another_var = if let Car {
    //     wheel_type: WheelType::Small(size),
    //     ..
    // } = a_var {
    //     format!("its a car with small wheels ({})", size)
    // } else if let Car {
    //     wheel_type: WheelType::Big(size),
    //     ..
    // } = a_var {
    //     format!("its a car with big wheels ({})", size)
    // };

    let another_var = match a_var {
        Car {wheel_type: WheelType::Small(size),..} => {
            format!("its a car with small wheels ({})", size)
        },
        Car {wheel_type: WheelType::Big(size),..} => {
            format!("its a car with big wheels ({})", size)
        }
    };


    print!("{:?}", another_var)
}