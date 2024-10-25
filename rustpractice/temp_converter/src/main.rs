const FARENHEIT_FP: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    let converion_ftc = (f - FARENHEIT_FP) * (5.0/9.0);
    return converion_ftc;
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    let conversion_ctf = (c * (9.0/5.0)) + FARENHEIT_FP;
    return conversion_ctf;
} 

fn main() {

    let mut f = 50.0;

    loop {
        println!("{} degrees Fahrenheit is {} degrees Celsius!", f, fahrenheit_to_celsius(f));
        f += 1.0;
        if f == 56.0 {
            break;
        }
    }

    let mut c = 20.0;
    
    loop {
        println!("{} degrees Celsius is {} degrees Fahrenheit!", c, celsius_to_fahrenheit(c));
        c += 1.0;
        if c == 26.0 {
            break;
        }
    }

}