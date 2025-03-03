fn main() {
    let config_const_values = {
        let config_path = std::env::args().nth(1).unwrap();
        let config_text = std::fs::read_to_string(&config_path).unwrap();
        config_text.parse::<toml::Value>().unwrap()
    };

    println!("Origianl:{:#?}", config_const_values);

    println!(
        "[Postgresql].Database:{}",
        config_const_values
            .get("postgresql")
            .unwrap()
            .get("database")
            .unwrap()
            .as_str()
            .unwrap()
    )
}
