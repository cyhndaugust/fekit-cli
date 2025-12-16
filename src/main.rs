use clap::Command;

fn main() {
    let _matches = Command::new("fekit")
        .version("0.1.0")
        .author("daniel")
        .about("Front-end toolkit CLI")
        .get_matches();
}
