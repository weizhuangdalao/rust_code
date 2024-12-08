use rand::seq::SliceRandom;
use zxcvbn::zxcvbn;

pub fn process_genpass(
    length: u8,
    uppercase: bool,
    lowercase: bool,
    number: bool,
    symbol: bool,
) -> anyhow::Result<()> {
    let mut rng = rand::thread_rng();
    let mut password = String::new();
    let mut chars = Vec::new();
    if uppercase {
        chars.extend_from_slice(b"QWERTYUPLKJHGFDSAZXCVBNM");
    };
    if lowercase {
        chars.extend_from_slice(b"qwertyuiopkjhgfdsazxcvbnm");
    };
    if number {
        chars.extend_from_slice(b"123456789");
    };
    if symbol {
        chars.extend_from_slice(b"_*&^%$#@!");
    }
    for _ in 0..length {
        let c = chars.choose(&mut rng).expect("chars wont be empty in this context");
        password.push(*c as char);
    }

    println!("password is {}", password);
    let estimate = zxcvbn(&password, &[]);
    println!("password score is {}", estimate.score()); // 3
    //模式 to do..
    Ok(())
}
