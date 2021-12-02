fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    let data = std::fs::read_to_string("data.txt")?;
    let lines = data.lines();

    for l in lines {
        let num: u32 = u32::from(l);
        println!("{}", num);
    }

    Ok(())
}
