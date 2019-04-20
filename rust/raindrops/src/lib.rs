

pub fn raindrops(n: u32) -> String {

    let mut result = "".to_string();

    if n % 3 == 0 {
        result = format!("{}", "Pling".to_string())
    }

    if n % 5 == 0  {
        result = format!("{}{}", result, "Plang".to_string())
    }

    if n % 7 == 0 {
        result = format!("{}{}", result, "Plong".to_string())
    }

    match result.len() {
        0 => format!("{}", n),
        _ => result.to_string()
    }
}
