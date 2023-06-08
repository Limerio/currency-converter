use clap::{Arg, ArgAction, Command};

pub fn cli() -> Command {
    let bin_name = "currency-converter";

    Command::new(bin_name)
        .bin_name(bin_name)
        .author("Limerio")
        .about("Currency converter")
        .arg(Arg::new("from").long("from").short('f').required(false))
        .arg(Arg::new("to").long("to").short('t').required(false))
        .arg(
            Arg::new("realtime")
                .long("realtime")
                .short('r')
                .required(false)
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("convert")
                .long("convert")
                .short('c')
                .required(false),
        )
}
