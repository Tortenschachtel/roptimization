use std::{fs::File, io::BufReader, collections::HashMap};

use serde::Deserialize;

use crate::{structs::{InputFile, WorkResult, InputData, Task}, util::{create_task_list, sum_of_size, optext_remove_file, optext_download_file, optext_run_task}};
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

const PARAM_EVOPARAMS: &str = "-params";

const fn default_generations() -> u32 { 50 }
const fn default_population() -> u32 { 10 }


#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct EvoParams {
  #[serde(default = "default_generations")]
  generations: u32,

  #[serde(default = "default_population")]
  population: u32,
}


#[derive(Clone)]
struct WorkingData {
  pub tasks: Vec<Task>,
  pub operations: Vec<String>,
  pub download_size: u64,
}

struct TaskPosition<'a> {
  task: &'a Task,
  pos_sum: usize,
  pos_mother: usize
}


pub fn evotask(data: InputData) -> Option<WorkResult> {
  // Construct the default evo params, then try to read the params file, if provided
  let mut population = default_population();
  let mut generations = default_generations();

  let args: Vec<String> = std::env::args().collect();
  for n in 0..args.len() {
    if &args[n] == PARAM_EVOPARAMS && (n + 1) < args.len() {
      let path = &args[n + 1];
      let file_result = File::open(path);
      match file_result {
        Err(err) => fail_reading_params(err.to_string()),
        Ok(file) => {
          let reader = BufReader::new(file);
          let data_result: Result<EvoParams, _> = serde_json::from_reader(reader);
    
          match data_result {
            Err(err) => fail_reading_params(err.to_string()),
            Ok(data) => {
              population = data.population;
              generations = data.generations;
            }
          }
        }
      }
    }
  }

  println!("generations: {}; population: {}", generations, population);

  let mut result: Option<WorkResult> = Option::None;

  // Build initial population
  let tasks = create_task_list(&data);
  let mut elements = prepare_initial_pop(&data, &tasks, population);
  elements.sort_by(|wd1, wd2| wd1.download_size.cmp(&wd2.download_size));

  for _ in 1..generations {
    let mut pop_new: Vec<WorkingData> = Vec::new();

    let half_population = elements.len() / 2;
    let max_mother: usize = std::cmp::max(std::cmp::min(2, elements.len()), half_population);
    let max_father: usize = elements.len();

    for imother in 0..max_mother {
      for ifather in 0..max_father {
        let child = generate_child(&elements[imother], &elements[ifather]);
        pop_new.push(calculate_operations(&data, &child));
      }
    }

    pop_new.sort_by(|wd1, wd2| wd2.download_size.cmp(&wd1.download_size));

    elements = Vec::new();
    // Alternatively just clone part of the cevtor, but there is literally no reason to clone the elements
    let target_number = std::cmp::min(population as usize, pop_new.len());
    for _ in 0..target_number {
      match pop_new.pop() {
        Option::None => (), // should never happen
        Option::Some(child) => elements.push(child)
      }
    }

    elements.sort_by(|wd1, wd2| wd1.download_size.cmp(&wd2.download_size));

    result = Option::Some(WorkResult {
      download_size: elements[0].download_size,
      operations: elements[0].operations.clone()
    });
  }

  result
}

 
fn fail_reading_params(error: String) {
  println!("Failed to read the parameters from file; continuing with default values. Previous error: {}", error);
}


fn prepare_initial_pop(data: &InputData, tasks: &Vec<Task>, population: u32) -> Vec<WorkingData> {
  let mut result: Vec<WorkingData> = Vec::new();
  let mut rng = thread_rng();

  for _ in 0..population {
    let mut vector = tasks[..].to_vec();
    vector.shuffle(&mut rng);
    result.push(calculate_operations(&data, &vector));
  }

  result
}


fn calculate_operations(data: &InputData, tasks: &Vec<Task>) -> WorkingData {
  let mut download_size: u64 = 0;
  let mut loaded_files: Vec<&InputFile> = Vec::new();
  let mut operations: Vec<String> = Vec::new();

  for index in 0..tasks.len() {
    let task = &tasks[index];

    // Check if either file for the task is loaded and calculate how much data must be loaded
    let first_loaded = loaded_files.iter().find(|file| file.name == task.first).is_some();
    let second_loaded = loaded_files.iter().find(|file| file.name == task.second).is_some();

    let first_file: &InputFile = data.files.iter().find(|file| file.name == task.first).expect("Failed to find first file");
    let second_file: &InputFile = data.files.iter().find(|file| file.name == task.second).expect("Failed to find first file");

    let mut add_files: Vec<&InputFile> = Vec::new();
    if !first_loaded {
      add_files.push(first_file);
    }
    
    if !second_loaded {
      add_files.push(second_file);
    }
    
    let required_space = sum_of_size(&add_files);
    let mut available_space = data.disk_size - sum_of_size(&loaded_files);

    // if we need more room remove some of the other files.
    if available_space < required_space {
      // Calculate when each file will be encountered the next time
      let mut first_encounter: HashMap<&String, usize> = HashMap::new();
      for i2 in index..tasks.len() {
        let task2 = &tasks[i2];
        if !first_encounter.contains_key(&task2.first) {
          first_encounter.insert(&task2.first, i2);
        }
        if !first_encounter.contains_key(&task2.second) {
          first_encounter.insert(&task2.second, i2);
        }
      }
      
      // first find files that are no longer needed and can be removed
      loaded_files.retain(|file| {
        if first_encounter.contains_key(&file.name) {
          return true;
        }
        
        operations.push(optext_remove_file(file));        
        false
      });

      available_space = data.disk_size - sum_of_size(&loaded_files);

      let mut max_index: usize = tasks.len();
      
      while available_space < required_space && max_index > 0 {
        max_index -= 1;

        let mut candidates: Vec<&InputFile> = Vec::new();
        for file in loaded_files.iter() {
          let encounter = first_encounter.get(&file.name);
          match encounter {
            None => (),
            Some(val) => if *val >= max_index {
              candidates.push(file);
            }
          }
        }

        // Order biggest first (so we can "pop" the smallest element)
        candidates.sort_by(|f1, f2| f2.size.cmp(&f1.size));

        let diff = required_space - available_space;
        let mut free_space: u64 = 0;

        while !candidates.is_empty() && free_space < diff {
          let can = candidates.pop().unwrap();
          loaded_files.retain(|f| f.name != can.name);
          operations.push(optext_remove_file(can));
          free_space += can.size;
        }

        available_space = data.disk_size - sum_of_size(&loaded_files);
      }
    }

    for file in add_files.iter() {
      operations.push(optext_download_file(file));
      download_size += file.size;

      loaded_files.push(file);
    }

    operations.push(optext_run_task(task));
  }


  WorkingData {
    tasks: tasks[..].to_vec(),
    operations,
    download_size
  }
}


fn generate_child(mother: &WorkingData, father: &WorkingData) -> Vec<Task> {
  // calculate position of task in both parents and sum it up
  // order tasks by sum and add them to the new vector in that order
  // for a bit of mutartion move a few tasks around a bit (not too much)

  let mut tmp: Vec<TaskPosition> = Vec::new();
  
  for imother in 0..mother.tasks.len() {
    let mtask = &mother.tasks[imother];
    
    for ifather in 0..father.tasks.len() {
      let ftask = &father.tasks[ifather];

      if ftask == mtask {
        tmp.push( TaskPosition {
          task: mtask,
          pos_sum: imother + ifather,
          pos_mother: imother
        })
      }
    }
  }

  tmp.sort_by(|tp1, tp2| {
    let cmp_sum = tp1.pos_sum.cmp(&tp2.pos_sum);
    if cmp_sum.is_eq() {
      return tp2.pos_mother.cmp(&tp2.pos_mother);
    }

    cmp_sum
  });

  let mut child: Vec<Task> = Vec::new();
  for tpos in tmp {
    child.push(tpos.task.clone());
  }

  let mut thread_rng = rand::thread_rng();
  let no = std::cmp::max(1, child.len() / 10);
  for _ in 0..no {
    let from = thread_rng.gen_range(0..child.len());
    let to = thread_rng.gen_range(0..child.len());

    child.swap(from, to);
  }

  child
}