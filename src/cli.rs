use clap::{App, Arg, ArgMatches};

#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub width: usize,
    pub height: usize,
    pub depth: usize,
    pub to_terminal: bool,
    pub gui: bool,
    pub method: Method,
    pub interactive: bool,
    pub subcommand: Subcommand,
    pub print_forest_stats: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Method {
    Basic,
    RayForest,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Subcommand {
    Normal,
    Benchmark(BenchmarkConfig),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BenchmarkConfig {
    pub runs: i32,
    pub filter_mode: bool,
}

pub fn configure_cli<'a, 'b>() -> App<'a, 'b> {
    let app = App::new("Rust Tracer")
        .version(option_env!("CARGO_PKG_VERSION").unwrap_or(""))
        .about("Ray Tracer")
        .arg(
            Arg::with_name("width")
                .long("width")
                .short("w")
                .takes_value(true)
                .default_value("512")
                .help("Set the width in pixels of the rendered image")
        )
        .arg(
            Arg::with_name("height")
                .long("height")
                .short("h")
                .takes_value(true)
                .default_value("512")
                .help("Set the height in pixels of the rendered image")
        )
        .arg(
            Arg::with_name("depth")
                .long("depth")
                .short("d")
                .takes_value(true)
                .default_value("8")
                .help("Set the maximum depth of reflections and transmissions which the ray tracer will follow when tracing a ray through the scene.")
        )
        .arg(
            Arg::with_name("method")
            .long("method")
            .takes_value(true)
            .default_value("basic")
            .help("Sets the rendering method that will be used: 1. Basic recursive rendering or 2. the RayForest method.")
            )
        .arg(
            Arg::with_name("to-terminal")
                .long("to-terminal")
                .short("t")
                .help("Render the scene as ASCII art to the terminal")
        )
        .arg(
            Arg::with_name("interactive")
            .long("interactive")
            .short("i")
            .help("When in CLI mode, this will block after each stage of the rendering pipeline and wait for the user before proceeding.  Useful for performance analysis and debugging.")
        )
        .arg(
            Arg::with_name("stats")
            .long("stats")
            .help("When using \"rayforest\" method, print out stats about the forest")
        )
        .subcommand(
            App::new("bench")
            .about("Runs benchmark tests to aid with performance testing and analysis")
            .arg(
                Arg::with_name("runs")
                .help("How many times to run the test")
                .long("runs")
                .short("-n")
                .default_value("10")
            )
            .arg(
                Arg::with_name("filter")
                .help("Test the ray forest render filter method")
                .long("filter")
                .short("f")
            )
        );

    #[cfg(target_os = "linux")]
    {
        app.arg(
            Arg::with_name("gui")
                .long("gui")
                .short("g")
                .help("Open a GUI for interacting with the ray tracer."),
        )
    }

    #[cfg(not(target_os = "linux"))]
    {
        app
    }
}

pub fn parse_args(args: &ArgMatches) -> Config {
    let width = args
        .value_of("width")
        .map(|s| s.parse::<usize>().expect("Expected integer for width"))
        .unwrap();
    let height = args
        .value_of("height")
        .map(|s| s.parse::<usize>().expect("Expected integer for height"))
        .unwrap();
    let depth = args
        .value_of("depth")
        .map(|s| s.parse::<usize>().expect("Expected integer for depth"))
        .unwrap();
    let to_terminal = args.is_present("to-terminal");
    let gui = args.is_present("gui");
    let interactive = args.is_present("interactive");
    let method = match args.value_of("method").map(|v| v.to_lowercase()) {
        None => Method::Basic,
        Some(x) => {
            if x == "rayforest" {
                Method::RayForest
            } else if x == "basic" {
                Method::Basic
            } else {
                panic!("Unexpected value provided for `--method`: {}", x);
            }
        }
    };
    let print_forest_stats = args.is_present("stats");

    let subcommand = args
        .subcommand_matches("bench")
        .map_or(Subcommand::Normal, |sub| {
            let runs = sub
                .value_of("runs")
                .map(|n| {
                    n.parse::<i32>()
                        .expect("Expected integer for number of runs")
                })
                .unwrap();
            let filter_mode = sub.is_present("filter");
            Subcommand::Benchmark(BenchmarkConfig { runs, filter_mode })
        });

    Config {
        width,
        height,
        depth,
        to_terminal,
        gui,
        method,
        interactive,
        subcommand,
        print_forest_stats,
    }
}
