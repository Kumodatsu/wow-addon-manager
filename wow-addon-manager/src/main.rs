mod net;
mod file;

fn main() {
    let config = file::config::read_config("config.yaml")
        .expect("Could not open configuration file.");
    println!("{:?}", config);
}
