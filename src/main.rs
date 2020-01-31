use clap::{crate_name, crate_version, App, Arg, ArgMatches};
use std::error::Error;
use utils::{dependencies, sh, Die};

fn main() {
    #[rustfmt::skip]
    let cli = App::new(crate_name!())
        .version(crate_version!())
        .about("Configure monitors using xrandr")
        .arg(Arg::with_name("TARGET")
             .help("The configuration to apply.")
             .possible_values(&["laptop", "desktop", "both"])
             .required_unless("src"))
        .arg(Arg::from("--src 'Prints this program's source to stdout'"))
        .get_matches();
    run(cli).unwrap_or_die();
}

fn run(cli: ArgMatches) -> Result<(), Box<dyn Error>> {
    if cli.is_present("src") {
        print!("/// main.rs\n{}", include_str!("main.rs"),);
        return Ok(());
    }
    dependencies(vec!["xrandr"])?;
    match cli.value_of("TARGET") {
        Some("laptop") => sh(r"xrandr --output HDMI2 --off
                 xrandr --output eDP1 --auto")?,
        Some("desktop") => sh(r"xrandr --output HDMI2 --auto --mode 3840x2160 --scale .5
                 xrandr --output eDP1 --off")?,
        Some("both") => sh(r"xrandr --output HDMI2 --auto --mode 3840x2160 --scale .5
                 xrandr --output eDP1 --auto")?,
        None | Some(_) => unreachable!("Clap requires & validates values"),
    };
    Ok(())
}
