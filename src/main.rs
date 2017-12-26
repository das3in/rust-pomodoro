#[macro_use]
extern crate chan;

#[macro_use]
extern crate clap;

use std::{thread, time};
use std::process::Command;

use clap::{Arg, App};

fn main() {
    let matches = App::new("r-modoro")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Pomodoro app in your terminal!")
        .arg(Arg::with_name("work-interval")
             .short("wi")
             .long("work-interval")
             .help("How long you want to work for?")
             .takes_value(true))
        .arg(Arg::with_name("break-interval")
             .short("bi")
             .long("break-interval")
             .help("How long should your short break be?")
             .takes_value(true))
        .arg(Arg::with_name("rest-interval")
             .short("ri")
             .long("rest-interval")
             .help("How long should your long breaks be?")
             .takes_value(true))
        .get_matches();

    let interval_param: u32 = matches
        .value_of("work-interval")
        .unwrap_or("1500000")
        .parse()
        .unwrap();
    let break_param: u32 = matches
        .value_of("break-interval")
        .unwrap_or("180000")
        .parse()
        .unwrap();
    let rest_param: u32 = matches
        .value_of("rest-interval")
        .unwrap_or("600000")
        .parse()
        .unwrap();

    let pomodoro = chan::tick_ms(interval_param);
    let short_break = chan::tick_ms(break_param / 3);

    let mut intervals_lapsed = 0;
    let mut break_minutes_remaining = 3;

    let break_message = format!("Time for a {}ms break", break_param);
    let rest_message = format!("Time for a longer, {}ms rest", rest_param);

    loop {
        chan_select! {
            pomodoro.recv() => {

                intervals_lapsed += 1;
                println!("{} lapsed", intervals_lapsed);

                if intervals_lapsed == 4 {
                    intervals_lapsed = 0;

                    Command::new("notify-send")
                        .arg(&rest_message)
                        .spawn()
                        .expect("Something went wrong with rest");

                    thread::sleep(time::Duration::from_millis(rest_param as u64));
                } else {
                    Command::new("notify-send")
                        .arg(&break_message)
                        .spawn()
                        .expect("Something went wrong");

                    thread::sleep(time::Duration::from_millis(break_param as u64));
                }
            }
        }
    }
}
