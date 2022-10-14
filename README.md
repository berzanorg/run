# ⚡ Run
A blazing-fast tool to manage end execute your scripts.

<p align="center">
  <a href="https://github.com/BerzanXYZ/run"><img src="https://raw.githubusercontent.com/BerzanXYZ/run/main/assets/run.png" alt="Logo" height=256></a>
</p>

#### Run:

- Can use [`scripts`](https://docs.npmjs.com/cli/v6/using-npm/scripts) of `package.json` and [`tasks`](https://deno.land/manual/tools/task_runner) of `deno.json`.
- Is blazing-fast, cuz it's written in Rust.
- Supports comments.
- Auto-sorts scripts alphabetically.
- Has smart error reporting.



<br/>



## Install
For macOS, Linux, or Windows Subsystem for Linux.
```sh
# to do
```



<br/>



## Usage


### Generate a `run.yaml` file.
> It will import scripts from `package.json` or `deno.json`, if any of them exists in the current directory.
```sh
run -i  # or `run --init`
```
<p align="left">
  <a href="https://github.com/BerzanXYZ/run"><img src="https://raw.githubusercontent.com/BerzanXYZ/run/main/assets/run.yaml.png" alt="Logo" height=200></a>
</p>



<br/>



### Print Scripts
> Command below displays all the scripts inside `run.yaml`.
```sh
run
```
<p align="left">
  <a href="https://github.com/BerzanXYZ/run"><img src="https://raw.githubusercontent.com/BerzanXYZ/run/main/assets/scripts.png" alt="Logo" height=128></a>
</p>


<br/>


### Run a Script
> You can run scripts by their name. You can also use an alias.
```sh
run c  # or `run compile`
```
<p align="left">
  <a href="https://github.com/BerzanXYZ/run"><img src="https://raw.githubusercontent.com/BerzanXYZ/run/main/assets/run_c.png" alt="Logo" height=128></a>
</p>

