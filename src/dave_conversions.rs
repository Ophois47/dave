pub fn fahrenheit_to_celsius(temp_in_f: f32) -> f32 {
    (temp_in_f - 32.0) / 1.8
}

pub fn celsius_to_fahrenheit(temp_in_c: f32) -> f32 {
    temp_in_c * 1.8 + 32.0
}

pub fn kelvin_to_fahrenheit(temp_in_k: f32) -> f32 {
    temp_in_k * 1.8 - 459.67
}

pub fn kelvin_to_celsius(temp_in_k: f32) -> f32 {
    temp_in_k - 273.15
}

pub fn fahrenheit_to_kelvin(temp_in_f: f32) -> f32 {
    (temp_in_f + 459.67) / 1.8
}

pub fn celsius_to_kelvin(temp_in_c: f32) -> f32 {
    temp_in_c + 273.15
}

pub fn pounds_to_kilos(pounds: f32) -> f32 {
    pounds / 2.205
}

pub fn kilos_to_pounds(kilos: f32) -> f32 {
    kilos * 2.205
}

pub fn mph_to_kph(mph: f32) -> f32 {
    mph * 1.609
}

pub fn kph_to_mph(kph: f32) -> f32 {
    kph / 1.609
}
