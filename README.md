# Model Based Simulation Framework
This project is my attempt to create an effective model-based simulation framework for the modern day. It employes modern languages including Rust and Python.

**This is an early draft of the framework. It is not yet ready for use.**

## Building
One major benefit is that the entire simulation is managed by Rust's highly supported Cargo package manager. The builds are highly modular, and allow for quick integration.

An interface tool simply titled 'sim' allows for idiomatic control over the entire process. Simply call `sim build` to invoke the cargo build system!
```shell
sim on  master [?] is 📦 v0.1.0 via 🦀 v1.62.1 
⬢ [Systemd] ❯ sim build
   Compiling simfrastructure v0.1.0 (/config/workspace/rust/sim/simfrastructure)
   Compiling force_effector v0.1.0 (/config/workspace/rust/sim/models/base/force_effector)
   Compiling eom v0.1.0 (/config/workspace/rust/sim/models/base/eom)
   Compiling models v0.1.0 (/config/workspace/rust/sim/models)
   Compiling sim v0.1.0 (/config/workspace/rust/sim)
    Finished dev [unoptimized + debuginfo] target(s) in 1.89s
```

## Running
Running the simulation is just as easy. Scenarios allow you to change the sim's behavior based on a set of inputs. When you run, specify a scenario!

```shell
sim on  master [?] is 📦 v0.1.0 via 🦀 v1.62.1 took 2s 
⬢ [Systemd] ❯ sim run basic
Generating runtime from /config/workspace/rust/sim/scenarios/basic/runconfig.py!
Simulation Description: A basic system to simulate!
Attempting to generate EOM
Attempting to generate ForceEffector
Model EOM { x: 0, y: 0, z: 100000000000000000, force_effectors: [], model_details: ModelDetails { order: 100, name: "EOM_0" } }
Model ForceEffector { fx: 0, fy: 0, fz: 0, model_details: ModelDetails { order: 120, name: "ForceEffector_1" } }
```

## Execution Steps
First, the simulation loads the runconfig.py file within the specified scenario. This file contains definitions for what models should be run, what their inputs should be, and how they should connect to each other.

Second, the simulation runtime generates native models from the definitions in the runconfig.py, filling in any references with a temporary value.

Third, once all models are created, references are filled in to point to the actual model.

Fourth, the run begins. I am not yet at this part...