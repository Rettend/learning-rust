fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }

    let mut count = 0;
    match count {
        1 => println!("One"),
        _ => {
            count += 1;
        }
    }

    let mut count = 0;
    if let 1 = count {
        println!("One");
    } else {
        count += 1;
    }
}
