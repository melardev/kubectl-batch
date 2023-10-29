# Introduction

This small application was created to be able to make work the following command:
`kubectl apply -f /my/path/*/jobs.yaml`.

It will just forward all the arguments given, except the paths to yaml files,
to `kubectl` binary, applied one by one to each yaml file passed as argument to the program.

# Run

Build
`cargo build --release`

- Run
  `./target/release/kubectl-batch apply -f /my/path/*/jobs.yaml`