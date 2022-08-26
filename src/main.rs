use clap::Parser;
use clipboard::{ClipboardContext, ClipboardProvider};
use uuid::Uuid;

fn main() {
    let args = Args::parse();

    let id = Uuid::new_v4();

    let formatted = match args.style {
        Style::CSharp => format!("[Guid({})]", id.hyphenated()),
    };
    println!("{}", formatted);

    if args.copy {
        let mut clipboard: ClipboardContext =
            ClipboardProvider::new().expect("Unable to get Clipboard reference");
        clipboard
            .set_contents(formatted)
            .expect("Unable to set Clipboard");
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(value_enum)]
    style: Style,
    #[clap(short, long, action)]
    copy: bool,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum Style {
    CSharp,
}
