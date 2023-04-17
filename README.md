# roptimization
## Table of content
- [roptimization](#roptimization)
  - [Table of content](#table-of-content)
  - [Introduction](#introduction)
  - [Technology and Implementation](#technology-and-implementation)
  - [How to use](#how-to-use)
  - [Command line parameters](#command-line-parameters)
  - [Examples](#examples)

## Introduction
This program is supposed to solve the following optimization problem:
- There are 10 files named A trough J with different sizes (see [data.json](./data.json)).
- For each combination of two different files a task needs to run.
- The available size of the file system where these files will be saved before they can run is limited to 20 GB (see also [data.json](./data.json))..

Find the order in which files should be downloaded, tasks executed and files deleted from the file system again, that ensures all tasks are completed while resulting in the smallest total download size.

**Note:** The program takes the liberty of assuming that task (A,B) is equivalent to task (B,A), so the latter is not considered separately. This simplifies the problem somewhat.

## Technology and Implementation
The program is written in Rust and uses Cargo as build system. Executing "cargo build" in the root folder should download all required dependencies and build the program.

There are multiple algorithms implemented to solve the problem:
- A brute force algorithm that takes exponentialy more time the larger the problem gets. It works fine if you only have 5 files but more than that means it will run for a long time.
- A simple genetic algorithm that can reach reasonable results.
- A naive algorithm that is probably getting the best possible value.

## How to use
To get a solution start the program in a terminal/console, optionally provide comand line parameters (see [below](#command-line-parameters)) and after a few seconds (or minutes, hours, even millenia, depending on the algoritm and the parameters used) the results will be printed to the console. Use the appropriate commands to route the output into a file if you want easier access to the results.

## Command line parameters
- `-data` => Provide a file path to a JSON-file that contains the data the program should work with. The data structure must be valid JSON, see [data.json](./data.json) for an example. If this parameter is not given the program tries to find the path "data.json" in its working directory.
- `-alg` => Provide the algorithm to use. Valid values are `brute` (a brute force algorithm), `evo` (a genetic algorithm) and `naive` (a naive algorithm for the problem) - see also the description of the different algoritms under section [Technology and Implementation](#technology-and-implementation). If this parameter is not provided the current default is the naive algorithm. If an unknown value if provided the program will simply return without finding a result.
- `-params` => Provide a file containing required parameters for an algorithm. Currently only the genetic algorithm requires additional parameters, to provide them add the path to the file [evoparams.json](./evoparams.json) as the value for this parameter. You can also provide any other file that follows the structure of [evoparams.json](./evoparams.json). Note that all values in that file (except for the root element) are optional and will be filled with default values if not provided, including in case the parameter `-params` is not provided at all.

## Examples
> roptimization

Run the optimization program using the data provided in the file `./data.json` and use the default algorithm

---
> roptimization -data ./my-data.json -alg evo -params ./evoparams.json

Run the optimization program using the data provided in the file `./my-data.json`, use the genetic algorith and use the parameters for that algorithm as provided in the file `./evoparams.json`

