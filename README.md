# OOO
Prompt Ollama from the command line either through the stdin or just as a normal prompt.<br>
You need to have [Ollama](https://github.com/jmorganca/ollama) installed in order to use ooo.

## Examples:
```shell
$ ls -lah | ooo I will give you an input from the terminal command ls -lah, can you take the filenames and use them as keys in a json format, and the value for each key is the permissions, far left
```
Or you can use the `--user` flag to specify another order of the input.
```shell
$ ls -lah | ooo --model llama2 --user "I'll give you an input from the terminal command 'ls -lah', can you take the filenames and use them as keys in a json format, and the value for each key is the permissions, far left"
```
#### Output:
```shell
{
".": "drwxr-xr-x",
"..": "drwxr-xr-x",
".git": "drwxr-xr-x",
".gitignore": "-rw-r--r--",
"Cargo.lock": "-rw-r--r--",
"Cargo.toml": "-rw-r--r--",
"README.md": "-rw-r--r--",
"src": "drwxr-xr-x",
"target": "drwxr-xr-x"
}
```
Or just do normal prompting.
```shell
$ ooo whats the tallest building\?
```
```shell
$ ooo --user "whats the tallest building?"
```
#### Output:
```shell
The tallest building in the world is the Burj Khalifa, located in Dubai, United Arab Emirates. Its height is 828 meters (2,716 feet)
```


## Install
```
brew tap nictap/tap
brew install ooo
```

## Options

| Flag          | Default           |
|---------------|-------------------|
| --user        | ''                |
| --system      | (see below)       |
| --model       | mistral           |
| --port        | 11434             |
| --url         | http://localhost  |


## System default
"You are a command-line program that takes an input and provides an output ONLY.
Give me only the output, without any additional labels (e.g., 'Output' or 'Result').
The output should be usable as input in another program that is not an LLM.
Avoid unnecessary chat.
No preamble, get straight to the point.
Generate a text response suitable for downstream processing by another program.
Do not change the content of the input unless specifically asked to.
Do not repeat back the input."

---

<sub><sup>(**ooo** isn't an acronym for anything, just easy to type. Lets say it's "Ollama -something")</sup></sub>
