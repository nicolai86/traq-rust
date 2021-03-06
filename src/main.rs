#![feature(collections)]
#![feature(convert)]

extern crate time;

use std::env;


extern crate getopts;
use getopts::Options;

extern crate glob;
use std::io::{ BufRead, BufReader };
use std::fs::File;
use std::path::Path;
use glob::glob;

#[derive(Debug)]
enum DateRequest {
  Month { year: i32, month: i32 },
  Day { year:i32, month:i32, day: i32 }
}

#[derive(Debug)]
struct TraqEntry {
  startsAt: i64,
  endsAt:   i64,
  tag:      String
}

#[derive(Debug)]
struct TraqEval {
  tag: String,
  duration: i64
}

fn parse_date(cli_opts: &getopts::Matches) -> DateRequest {
  match cli_opts.opt_str("d").and_then(|d| {
      time::strptime(d.as_str(), "%Y-%m-%d").and_then(|t| {
        Ok(DateRequest::Day{
          year:  t.tm_year + 1900_i32,
          month: t.tm_mon + 1,
          day:   t.tm_mday,
        })
      }).ok()
    }) {
    Some(d) => return d,
    None => (),
  };

  let now = time::now();

  let mut year  = now.tm_year + 1900_i32;
  let mut month = now.tm_mon + 1;

  cli_opts.opt_str("m").and_then(|m| {
    m.parse::<i32>().and_then(|m|{
      month = m;
      Ok(m)
    }).ok()
  });

  cli_opts.opt_str("y").and_then(|y|{
    y.parse::<i32>().and_then(|y|{
      year = y;
      Ok(y)
    }).ok()
  });

  time::strptime(format!("{}-{}-{}", year, month, 1).as_str(), "%Y-%m-%d").map_err(|_| {
    year = now.tm_year + 1900_i32;
    month = now.tm_mon + 1;
  });

  return DateRequest::Month{
    year:  year,
    month: month,
  };
}

fn project_path(project: &str, date: &DateRequest) -> String {
  let mut data_dir = String::from_str( env!("TRAQ_DATA_DIR") );
  data_dir.push_str("/");
  data_dir.push_str(project);
  data_dir.push_str("/");
  match *date {
    DateRequest::Month{ year: y, month:_} => {
      data_dir.push_str(y.to_string().as_str());
    }
    DateRequest::Day{ year:y, month:_, day:_} => {
      data_dir.push_str(y.to_string().as_str());
    }
  }
  return data_dir;
}

fn evaluate(project: &str, date: &DateRequest, running: bool) {
  match *date {
    DateRequest::Month{ year: y, month: m} => {
      for entry in glob(format!("{}/{}-{:02}-*", project_path(project, &date).as_str(), y, m).as_str()).unwrap().filter_map(Result::ok) {
        let entries = evaluate_file(entry.as_path());
        println!("{:?}", entries);
      }
    }
    DateRequest::Day{ year:y, month:m, day: d} => {
      let entries = evaluate_file(Path::new(format!("{}/{}-{:02}-{:02}", project_path(project, &date).as_str(), y, m, d).as_str()));
      println!("{:?}", entries);
    }
  }
}

fn evaluate_file(path: &Path) -> Vec<TraqEntry> {
  let file = BufReader::new(File::open(&path).unwrap());
  let mut vec = Vec::<TraqEntry>::new();
  let mut current: TraqEntry = TraqEntry{ startsAt: 0i64, endsAt: 0i64, tag: String::from_str("") };
  for line in file.lines() {
    let l = line.unwrap();
    let v: Vec<&str> = l.split(';').collect();
    let tag = v[1];
    if current.startsAt == 0i64 {
      current.tag = String::from_str(tag);

      match time::strptime(v[0], "%a %b %e %T %z %Y") {
        Ok(t) => { current.startsAt = t.to_timespec().sec }
        Err(e) => {
          panic!("failed to parse {}", path.display());
        }
      }
      continue
    }

    if tag == "stop" {
      match time::strptime(v[0], "%a %b %e %T %z %Y") {
        Ok(t) => { current.endsAt = t.to_timespec().sec }
        Err(e) => {
          panic!("failed to parse {}", path.display());
        }
      };
      vec.push(current);
      current = TraqEntry{ startsAt: 0i64, endsAt: 0i64, tag: String::from_str("") };
    } else {
      let mut n: TraqEntry = TraqEntry{ startsAt: 0i64, endsAt: 0i64, tag: String::from_str("") };
      match time::strptime(v[0], "%a %b %e %T %z %Y") {
        Ok(t) => {
          n.startsAt = t.to_timespec().sec;
          current.endsAt = t.to_timespec().sec }
        Err(e) => {
          panic!("failed to parse {}", path.display());
        }
      };
      vec.push(current);
      current =n;
    }
  }
  return vec
}

fn print_file(path: &Path) {
  match File::open(&path) {
    Ok(file) => {
      let file = BufReader::new(file);
      for line in file.lines() {
        print!("{}\n", line.unwrap());
      }
      print!("%%\n");
    }
    Err(e) => { println!("{}: {:?}", path.display(), e); }
  }
}

fn print_date(project: &str, date: &DateRequest) {
  match *date {
    DateRequest::Month{ year: y, month: m} => {
      for entry in glob(format!("{}/{}-{:02}-*", project_path(project, &date).as_str(), y, m).as_str()).unwrap().filter_map(Result::ok) {
        print_file(entry.as_path());
      }
    }
    DateRequest::Day{ year:y, month:m, day: d} => {
      print_file(Path::new(format!("{}/{}-{:02}-{:02}", project_path(project, &date).as_str(), y, m, d).as_str()))
    }
  }
}

#[cfg(not(test))]
fn main() {
  let args: Vec<String> = env::args().collect();

  let mut opts = Options::new();
  opts.optopt("m",  "", "print tracked times for a given month"   , "MONTH");
  opts.optopt("y",  "", "print tracked times for a given year"    , "YEAR");
  opts.optopt("p",  "timestamps", "print data for a given project", "PROJECT");
  opts.optopt("d",  ""          , "print tracked times for a given date", "DATE");
  opts.optflag("e", "evaluate"  , "evaluate times by tag");
  opts.optflag("r", "running"   , "include active tags in evaluation");

  let cli_opts = match opts.parse(&args[1..]) {
      Ok(m) => { m }
      Err(f) => { panic!(f.to_string()) }
  };

  let project = cli_opts.opt_str("p").unwrap_or( String::from_str( "timestamps" ) );

  let date = parse_date(&cli_opts);

  if cli_opts.opt_present("e") {
    let running_evaluation = cli_opts.opt_present("r");
    evaluate( project.as_str(), &date, running_evaluation );
  } else {
    let command = if cli_opts.free.len() > 0 {
      cli_opts.free[0].as_str()
    } else {
      ""
    };

    if command == "" {
      print_date( project.as_str(), &date);
    } else {
      println!("store! {}", command);
    }
  }
}
