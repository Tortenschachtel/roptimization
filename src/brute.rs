use crate::{structs::{InputFile, WorkResult, InputData, Task}, util::{create_task_list, sum_of_size, optext_remove_file, optext_download_file, optext_run_task, can_do_task, return_smaller_result}};

#[derive(Clone)]
pub struct WorkingData<'a> {
  pub remaining_tasks: Vec<Task>,
  pub operations: Vec<String>,
  pub loaded_files: Vec<&'a InputFile>,
  pub download_size: u64,
}

fn prepare_woring_data(data: &InputData) -> WorkingData {
  let remaining_tasks: Vec<Task> = create_task_list(data);

  WorkingData {
    remaining_tasks,
    operations: Vec::new(),
    loaded_files: Vec::new(),
    download_size: 0,
  }
}

pub fn brute(data: InputData) -> Option<WorkResult> {
  let working_data: WorkingData = prepare_woring_data(&data);

  let mut result: Option<WorkResult> = Option::None;
  let mut stack: Vec<WorkingData> = Vec::new();

  stack.push(working_data);

  while !stack.is_empty() {
    let wdata: WorkingData = stack.pop().expect("Failed to pop non-empty stack.");

    if wdata.remaining_tasks.is_empty() {
      let new_result: WorkResult = WorkResult {
        download_size: wdata.download_size,
        operations: wdata.operations[..].to_vec()
      };

      result = return_smaller_result(result, new_result);
    } else {
      for task in &wdata.remaining_tasks {
        // Check if either file for the task is loaded and calculate how much data must be loaded
        let first_loaded = wdata.loaded_files.iter().find(|file| file.name == task.first).is_some();
        let second_loaded = wdata.loaded_files.iter().find(|file| file.name == task.second).is_some();

        let first_file: &InputFile = data.files.iter().find(|file| file.name == task.first).expect("Failed to find first file");
        let second_file: &InputFile = data.files.iter().find(|file| file.name == task.second).expect("Failed to find first file");

        let mut add_files: Vec<&InputFile> = Vec::new();
        if !first_loaded {
          add_files.push(first_file);
        }
        
        if !second_loaded {
          add_files.push(second_file);
        }
        
        let additional_space = sum_of_size(&add_files);
        let loaded_size = sum_of_size(&wdata.loaded_files);
        let available_space = data.disk_size - loaded_size;

        if available_space >= additional_space {
          // Simple case: add the files to the loaded files, check all tasks and add a new WorkingData to the stack
          stack.push(create_working_data(&wdata, &add_files, wdata.operations[..].to_vec(), wdata.loaded_files[..].to_vec()));
        } else {
          // Remove as many files as necessary to make room for the required files
          // DO NOT remove the required files that are already loaded, if any
          // Create a new WorkingData for any combination and add it to the stack.
          let required_space: u64 = additional_space - available_space;
          let mut valid_files: Vec<&InputFile> = Vec::new();
          for file in &wdata.loaded_files {
            if file.name != task.first && file.name != task.second {
              valid_files.push(file);
            }
          }

          // Calculate all minimal combinations of files that are big enough to satisfy required_space
          let valid_combinations: Vec<Vec<&InputFile>> = collect_combinations(required_space, &valid_files);

          for vec in &valid_combinations {
            let mut remaining_files:Vec<&InputFile> = Vec::new();
            for file in &wdata.loaded_files {
              if vec.iter().find(|f| file.name == f.name).is_none() {
                remaining_files.push(file);
              }
            }

            let mut operations = wdata.operations[..].to_vec();
            for file in vec {
              operations.push(optext_remove_file(file));
            }

            stack.push(create_working_data(&wdata, &add_files, operations, remaining_files));
          }
        }
      }
    }
  }

  result
}


fn collect_combinations<'a>(required_space: u64, valid_files: &Vec<&'a InputFile>) -> Vec<Vec<&'a InputFile>> {
  let mut incubator: Vec<Vec<&InputFile>> = Vec::new();
  let mut result: Vec<Vec<&InputFile>> = Vec::new();

  for file in valid_files {
    incubator.push(vec!(file));
  }

  while !incubator.is_empty() {
    let vec: Vec<&InputFile> = incubator.pop().expect("failed to pop non-empty stack");
    let vec_size: u64 = sum_of_size(&vec);

    if vec_size >= required_space {
      result.push(vec);
    } else {
      for file in valid_files {
        if vec.iter().find(|f| file.name == f.name).is_none() {
          let mut vec_new = vec[..].to_vec();
          vec_new.push(file.clone());

          incubator.push(vec_new);
        } 
      }
    }
  }

  result
}


/** Add the given files from add_files to the loaded_files (including all side effects) and return a new WorkingData */
fn create_working_data<'a>(original: &WorkingData, add_files: &Vec<&'a InputFile>, mut operations: Vec<String>, mut loaded_files: Vec<&'a InputFile>) -> WorkingData<'a> {
  let mut download_size: u64 = original.download_size;

  for file in add_files {
    operations.push(optext_download_file(file));
    loaded_files.push(file);
    download_size += file.size;
  }

  let mut remaining_tasks = Vec::new();
  for task in &original.remaining_tasks {
    if can_do_task(&loaded_files, task) {
      operations.push(optext_run_task(task));
    } else {
      remaining_tasks.push(task.clone());
    }
  }

  WorkingData {
    operations,
    loaded_files,
    download_size,
    remaining_tasks,
  }
}
