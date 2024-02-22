use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "yuca", version = "0.1.0", author = "Yuca Organizations", about = "A CLI tool for generating files")]
struct Yuca {

    #[clap(subcommand)]
    cmd: Command,
}

#[derive(Parser, Debug)]
enum Command {
    #[clap(about = "Generate a use case file")]
    GenerateUseCase(GenerateCommand),
    #[clap(about = "Generate a controller file")]
    GenerateController(GenerateCommand),
    #[clap(about = "Generate any other type of file")]
    Generate(GenerateCommand),
}

#[derive(Parser, Debug)]
struct GenerateCommand {
    #[clap(short, long)]
    name: String,
}

pub fn run() {
    let yuca: Yuca = Yuca::parse();
    match yuca.cmd {
        Command::GenerateUseCase(gen) => generate_file("useCase", &gen.name),
        Command::GenerateController(gen) => generate_file("controller", &gen.name),
        Command::Generate(gen) => generate_file("file", &gen.name),
    }
}

fn generate_file(file_type: &str, name: &str) {
    println!("Generating {} file '{}'", file_type, name);
    // Add your file generation logic here
}
