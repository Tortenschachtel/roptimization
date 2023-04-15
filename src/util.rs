use crate::structs::{InputData, InputFile, Task, WorkResult};

pub fn create_task_list(data: &InputData) -> Vec<Task> {
  let mut result: Vec<Task> = Vec::new();

  let mut a: usize = 0;
  
  while a < data.files.len() {
    let mut b: usize = a + 1;

    while b < data.files.len() {
      let task = Task { first: data.files[a].name.clone(), second: data.files[b].name.clone() };
      result.push(task);

      b = b + 1;
    }

    a = a + 1;
  }
  
  result
}

pub fn sum_of_size(files: &Vec<&InputFile>) -> u64 {
  files.iter().fold(0u64, |acc, x| acc + x.size)
}

/** Return true if both files of a task are in the given list of loaded_files */
pub fn can_do_task(loaded_files: &Vec<&InputFile>, task: &Task) -> bool {
  loaded_files.iter().find(|f| task.first == f.name).is_some() && loaded_files.iter().find(|f| task.second == f.name).is_some()
}

/** Returns an Option<WorkResult>, depending on the input data:
 *  If old_result is Option::None or if old_result has a larger download_size than new_result return Some(new_result); otherwise return old_result
 */
pub fn return_smaller_result(old_result: Option<WorkResult>, new_result: WorkResult) -> Option<WorkResult> {
  if old_result.is_none() || old_result.as_ref().map(|r| r.download_size).unwrap() > new_result.download_size {
    return Some(new_result);
  }

  old_result
}

/** Create a text for the operations list that the given file has been downloaded */
pub fn optext_download_file(file: &InputFile) -> String {
  format!("+ Download file {}", file.name)
}

/** Create a text for the operations list that the given file has been removed from the disk */
pub fn optext_remove_file(file: &InputFile) -> String {
  format!("- Remove file {} from disk", file.name)
}

/** Create a text for the operations list that the given task ran */
pub fn optext_run_task(task: &Task) -> String {
  format!("> Run task ({}, {})", task.first, task.second)
}
