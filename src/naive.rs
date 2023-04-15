use crate::{structs::{InputFile, WorkResult, InputData, Task}, util::{create_task_list, sum_of_size, optext_remove_file, optext_download_file, optext_run_task}};

#[derive(Clone, PartialEq, Eq)]
pub struct TaskSize {
  first: String,
  second: String,
  size: u64,
}

pub fn naive(data: InputData) -> Option<WorkResult> {
  let mut loaded_files: Vec<&InputFile> = Vec::new();
  let mut tasks: Vec<Task> = create_task_list(&data);

  let mut operations: Vec<String> = Vec::new();
  let mut download_size: u64 = 0;  

  while !tasks.is_empty() {
    let mut required_files_first: Vec<&InputFile> = Vec::new();
    for file in &data.files {
      if tasks.iter().any(|task| task.first == file.name || task.second == file.name) {
        required_files_first.push(file);
      }
    }
    required_files_first.sort_by(|f1, f2| f2.size.cmp(&f1.size));

    let first_file = required_files_first[0];
    let first_loaded = loaded_files.iter().any(|file| file.name == first_file.name);

    let mut required_files_second: Vec<&InputFile> = Vec::new();
    for file in data.files.iter() {
      if tasks.iter().any(|task| task.first == file.name && task.second == first_file.name || task.first == first_file.name && task.second == file.name) {
        required_files_second.push(file);
      }
    }
    required_files_second.sort_by(|f1, f2| f2.size.cmp(&f1.size));

    let second_file = required_files_second[0];
    let second_loaded = loaded_files.iter().any(|file| file.name == second_file.name);

    let mut add_files: Vec<&InputFile> = Vec::new();
    if !first_loaded {
      add_files.push(first_file);
    }
    
    if !second_loaded {
      add_files.push(second_file);
    }
    
    // Kick out files that are never again needed
    loaded_files.retain(|file| {
      if !required_files_first.iter().any(|f2| f2.name == file.name) {
        operations.push(optext_remove_file(file));
        return false;
      }

      true
    });

    let required_space = sum_of_size(&add_files);
    let mut available_space = data.disk_size - sum_of_size(&loaded_files);

    while required_space > available_space && !loaded_files.is_empty() {
      let mut candidates: Vec<&InputFile> = Vec::new();
      for file in loaded_files.iter() {
        if file.name != first_file.name && file.name != second_file.name {
          candidates.push(file);
        }
      }
      candidates.sort_by(|a, b| a.size.cmp(&b.size));

      loaded_files.retain(|file| {
        if file.name == candidates[0].name {
          operations.push(optext_remove_file(file));
          return false;
        }

        true
      });
      available_space = data.disk_size - sum_of_size(&loaded_files);
    }

    for file in add_files.iter() {
      operations.push(optext_download_file(file));
      download_size += file.size;

      loaded_files.push(file);
    }

    tasks.retain(|task| {
      if loaded_files.iter().any(|file| file.name == task.first) && loaded_files.iter().any(|file| file.name == task.second) {
        operations.push(optext_run_task(task));
        return false;
      }

      true
    }); 
  }
  
  Option::Some(WorkResult {
    operations,
    download_size
  })
}