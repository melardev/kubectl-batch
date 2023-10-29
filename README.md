# Introduction

This small application was created to be able to make work the following command:<br/>
`kubectl apply -f /my/path/*/jobs.yaml`.

It will just forward all the arguments given, except the paths to yaml files,
to `kubectl` binary, applied one by one to each yaml file passed as argument to the program.

# Run

- Build<br/>
  `cargo build --release`

- Run<br/>
  `./target/release/kubectl-batch apply -f /my/path/*/jobs.yaml`