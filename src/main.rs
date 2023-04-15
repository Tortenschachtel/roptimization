mod util;
mod structs;

mod brute;
mod evotask;
mod naive;

use std::{io::{Result, BufReader}, time::Instant, fs::File};
use structs::{ InputData, WorkResult };

use brute::brute;
use evotask::evotask;
use naive::naive;

const PARAM_PATH: &str = "-data";
const PARAM_ALG: &str = "-alg";

const DEFAULT_PATH: &str = ".\\data.json";

const OPTION_BRUTE: &str = "brute";
const OPTION_EVOTASK: &str = "evo";
const OPTION_NAIVE: &str = "naive";


fn main() -> Result<()> {
  let mut path: &str = DEFAULT_PATH; 
  let mut opt: &str = OPTION_NAIVE;

  let args: Vec<String> = std::env::args().collect();
  for n in 0..args.len() {
    if &args[n] == PARAM_PATH && (n + 1) < args.len() {
      path = &args[n + 1];
    } else if &args[n] == PARAM_ALG && (n + 1) < args.len() {
      opt = &args[n + 1];
    }
  }

  let input_data: InputData = read_data(path)?;
  let mut smallest_result: Option<WorkResult> = Option::None;

  let start = Instant::now();

  if opt == OPTION_BRUTE {
    smallest_result = brute(input_data);
  } else if opt == OPTION_EVOTASK {
    smallest_result = evotask(input_data);
  } else if opt == OPTION_NAIVE {
    smallest_result = naive(input_data);
  }

  let elapsed = start.elapsed();
  println!("Done! Took: {:.2?}", elapsed);

  if smallest_result.is_some() {
    print_work_result(&smallest_result.unwrap());
  } else {
    println!("No result found.");
  }

  println!("\u{7}");

  Ok(())
}


pub fn read_data(path: &str) -> Result<InputData> {
  let file = File::open(path)?;
  let reader = BufReader::new(file);

  let data: InputData = serde_json::from_reader(reader)?;
  Ok(data)
}


/** Find the result with the smallest download size. If multiple results have the same size return the first. */
pub fn find_smallest(results: &Vec<WorkResult>) -> Option<WorkResult> {
  if results.is_empty() {
    return Option::None;
  }

  let mut result: &WorkResult = &results[0];

  for tmp in results {
    if tmp.download_size < result.download_size {
      result = tmp;
    }
  }

  Some(result.clone())
}


/** Print the total download size and all operations from the given result. */
pub fn print_work_result(result: &WorkResult) {
  println!("Total downloaded data: {}", result.download_size);
  println!("Operations:");

  for op in &result.operations {
    println!("{op}"); // Why does it *have* to be a literal?
  }
}
