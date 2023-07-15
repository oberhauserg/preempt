use chrono::{Duration, NaiveTime, Weekday};
use clap::{App, Arg, ArgMatches, SubCommand};
use preempt::context::Context;
use preempt::model::{load, save, PreemptApp};
use preempt::schedule::print_schedule;
use preempt::task::Task;
use preempt::timeblock::TimeBlock;

fn build_add_task_arg(app: App) -> App {
    app.subcommand(
        SubCommand::with_name("add-task")
            .about("Adds a new task")
            .arg(
                Arg::with_name("name")
                    .long("name")
                    .required(true)
                    .help("The name of the task")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("duration")
                    .long("duration")
                    .help("The duration of the task in minutes")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("context")
                    .long("context")
                    .help("The context of the task")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("priority")
                    .long("priority")
                    .help("The priority of the task (0-10)")
                    .takes_value(true),
            ),
    )
}

fn build_add_context_arg(app: App) -> App {
    app.subcommand(
        SubCommand::with_name("add-context")
            .about("Adds a new context")
            .arg(
                Arg::with_name("name")
                    .long("name")
                    .required(true)
                    .help("The name of the context")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("days")
                    .long("days")
                    .required(true)
                    .help("The days of the week for the context. Excepts Sun, Mon, Tue, Wed, Thu, Fri, Sat day codes.")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("start")
                    .long("start")
                    .required(true)
                    .help("The start time for the context")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("end")
                    .long("end")
                    .required(true)
                    .help("The end time for the context")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("transition")
                    .long("transition")
                    .required(false)
                    .help("The transition time between contexts")
                    .takes_value(true).validator(|x| {
                        x.parse::<i32>()
                            .map(|_| ())
                            .map_err(|_| String::from("The value must be an integer"))
                    }),
            ),
    )
}

fn build_show_context_arg(app: App) -> App {
    app.subcommand(
        SubCommand::with_name("show-context")
            .about("Shows details about a specific context")
            .arg(
                Arg::with_name("name")
                    .required(true)
                    .help("The name of the context"),
            ),
    )
}

fn build_timeline_arg(app: App) -> App {
    app.subcommand(
        SubCommand::with_name("timeline")
            .about("Creates and shows a timeline incorporating the current tasks."),
    )
}

fn handle_add_task(matches: &ArgMatches, app: &mut PreemptApp) -> Result<(), &'static str> {
    if let Some(sub_m) = matches.subcommand_matches("add-task") {
        let name = sub_m.value_of("name").unwrap(); // safe to unwrap because it's required

        let context: Option<&str> = sub_m.value_of("context");

        if context.is_some() {
            match app.get_context(&context.unwrap().to_string()) {
                Some(_) => {}
                None => {
                    return Err("Context doesn't exist.");
                }
            }
        }

        let priority = match sub_m.value_of("priority") {
            Some(pri) => pri.parse::<i32>().unwrap(),
            None => 1,
        };

        let a_task = match sub_m.value_of("duration") {
            Some(duration) => Task::new_with_duration(
                name.to_string(),
                name.to_string(),
                priority,
                false,
                Duration::minutes(duration.parse::<i64>().unwrap()),
                match context {
                    Some(name) => Some(name.to_string()),
                    None => None,
                },
            ),
            None => Task::new(
                name.to_string(),
                name.to_string(),
                priority,
                false,
                match context {
                    Some(name) => Some(name.to_string()),
                    None => None,
                },
            ),
        };

        match app.add_task(a_task) {
            Ok(_) => {}
            Err(error) => return Err(error),
        };
        return Ok(());
    }
    Ok(())
}

fn handle_add_context(matches: &ArgMatches, app: &mut PreemptApp) {
    if let Some(sub_m) = matches.subcommand_matches("add-context") {
        let name = sub_m.value_of("name").unwrap(); // safe to unwrap because it's required

        let days = sub_m
            .value_of("days")
            .unwrap()
            .split(',')
            .map(|d| d.parse::<Weekday>()) // Implement a function to convert string to Weekday
            .collect::<Result<Vec<Weekday>, _>>()
            .unwrap_or_else(|_| vec![]); // default to empty vector if parsing fails

        let start = NaiveTime::parse_from_str(sub_m.value_of("start").unwrap(), "%H:%M") // safe to unwrap because it's required
            .unwrap_or_else(|_| {
                NaiveTime::from_hms_opt(0, 0, 0).expect("Failed to create default start time")
            }); // default to midnight if parsing fails

        let end = NaiveTime::parse_from_str(sub_m.value_of("end").unwrap(), "%H:%M") // safe to unwrap because it's required
            .unwrap_or_else(|_| {
                NaiveTime::from_hms_opt(0, 0, 0).expect("Failed to create default end time")
            }); // default to midnight if parsing fails

        let transition = sub_m
            .value_of("transition")
            .map(|t| Duration::minutes(t.parse().unwrap_or(0)))
            .unwrap_or_else(|| Duration::minutes(0)); // default to 0 minutes if parsing fails or not provided

        let new_context = Context::new(name, days, start, end, transition);
        match app.add_context(new_context) {
            Ok(_) => {}
            Err(error) => {
                println!("{}", error)
            }
        };
    }
}

fn handle_show_context(matches: &ArgMatches, app: &mut PreemptApp) {
    if let Some(sub_m) = matches.subcommand_matches("show-context") {
        let name = sub_m.value_of("name").unwrap(); // safe to unwrap because it's required

        match app.get_context(&name.to_string()) {
            Some(context) => context.print(),
            None => {
                println!("No context by the name '{name}'");
            }
        }
    }
}

fn handle_timeline(matches: &ArgMatches, app: &PreemptApp) {
    if let Some(sub_m) = matches.subcommand_matches("timeline") {
        print_schedule(app.build_schedule());
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = App::new("preempt").about("A scheduler for humans.");
    let app = build_add_task_arg(app);
    let app = build_add_context_arg(app);
    let app = build_show_context_arg(app);
    let app = build_timeline_arg(app);
    let matches = app.get_matches();

    let mut preempt_app: PreemptApp = match load() {
        Ok(data) => data,
        Err(e) => {
            println!("Failed to load data: {}", e);
            // Create a new, empty PreemptApp if loading fails
            PreemptApp::new()
        }
    };

    match handle_add_task(&matches, &mut preempt_app) {
        Ok(_) => (),
        Err(error) => println!("{}", error),
    }
    handle_add_context(&matches, &mut preempt_app);
    handle_show_context(&matches, &mut preempt_app);
    handle_timeline(&matches, &preempt_app);

    match save(&preempt_app) {
        Ok(_) => (),
        Err(error) => println!("Error saving data: {}", error),
    }

    Ok(())
}
