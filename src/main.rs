extern crate traq;
extern crate time;
extern crate getopts;

use getopts::{optopt,optflag,getopts};

// use std::io::BufferedReader;
// use std::io::File;
use std::os;

fn parse_date(matches: &getopts::Matches) -> (i32, i32, i32) {
  let now = time::now();

  let mut year  = now.tm_year + 1900_i32;
  let mut month = now.tm_mon + 1;
  let mut day   = now.tm_mday;

  month = match matches.opt_str("m") {
    Some(m) => {
      match from_str(m.trim()) {
        Some(m) => {
          day = -1;
          m
        }
        None    => { month }
      }

    }
    None => { month }
  };

  year = match matches.opt_str("y") {
    Some(y) => {
      match from_str(y.trim()) {
        Some(y) => { y }
        None    => { year }
      }
    }
    None => { year }
  };

  match matches.opt_str("d") {
    Some(d) => {
      match time::strptime(d.as_slice(), "%Y-%m-%d") {
        Ok(t) => {
          month = t.tm_mon + 1;
          year  = t.tm_year + 1900_i32;
          day   = t.tm_mday;
        }
        Err(_) => {}
      }
    }
    None => {}
  }
  return (year, month, day);
}

fn project_path(project: &str, year: i32) -> String {
  let mut data_dir = String::from_str( env!("TRAQ_DATA_DIR") );
  data_dir.push_str("/");
  data_dir.push_str(project);
  data_dir.push_str("/");
  data_dir.push_str(year.to_string().as_slice());
  return data_dir;
}

fn evaluate(project: &str, year: i32, month: i32, day: i32, running: bool) {

}

fn print_month(project: &str, year: i32, month: i32) {

}

fn print_date(project: &str, year: i32, month: i32, day: i32) {

}

#[cfg(not(test))]
fn main() {
  let args: Vec<String> = os::args();
  // let program = args[0].clone();

  let opts = [
    optopt("m",  "", "print tracked times for a given month"   , "MONTH"),
    optopt("y",  "", "print tracked times for a given year"    , "YEAR"),
    optopt("p",  "timestamps", "print data for a given project", "PROJECT"),
    optopt("d",  ""          , "print tracked times for a given date", "DATE"),
    optflag("e", "evaluate"  , "evaluate times by tag"),
    optflag("r", "running"   , "include active tags in evaluation"),
  ];

  let matches: getopts::Matches = match getopts(args.tail(), opts) {
    Ok(m)  => { m }
    Err(f) => { panic!(f.to_string()) }
  };

  let project_match = matches.opt_str("p");
  let project = match project_match {
    Some(ref project) => { project.as_slice() }
    None              => { "timestamps" }
  };

  let (year, month, day) = parse_date(&matches);

  if matches.opt_present("e") {
    let running_evaluation = matches.opt_present("r");
    evaluate( project, year, month, day, running_evaluation );
  } else {

    let command = if matches.free.len() > 0 {
      matches.free[0].as_slice()
    } else {
      ""
    };

    if command == "" {
      println!("print!")
      if day == -1 {
        println!("month!")
        print_month( project, year, month );
      } else {
        println!("day!")
        print_date( project, year, month, day );
      }
    } else {
      println!("store! {}", command)
    }
  }

  println!("Hello World!");
  println!("project: {}", project);
  println!("project: {}", project_path(project, year));
  println!("y/m/d: {}/{}/{}", year, month, day);

  // let path = Path::new("src/lib.rs");
  // let mut file = BufferedReader::new(File::open(&path));
  // let lines: Vec<String> = file.lines().map(|x| x.unwrap()).collect();
}
