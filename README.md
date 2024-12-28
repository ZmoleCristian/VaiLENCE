# 🔫 VaiLENCE 🤖

## Description

VaiLENCE is a command-line utility designed for content moderation by leveraging the OpenAI API. The primary function of VaiLENCE is to evaluate text data for potentially harmful content, specifically focusing on categories such as violence and threats. It reads text data from either files or standard input, processes this data in manageable chunks, and outputs the results, including severity scores, in JSON format.

VaiLENCE is ideal for developers, moderators, and organizations looking to automate the detection of harmful content in large volumes of text data.

## 🪄 Features

- Customizable Severity Threshold: 
  - Users can define a minimum severity score, which allows the filtering of results based on the desired sensitivity level.
- Flexible Input/Output Options: 
  - Accepts input from a file or standard input and allows output to be directed to files or standard output.
- Verbose Output: 
  - Provides an option to display output results directly in the console even when output files are specified.
- Chunk-Based Processing: 
  - Users can configure the size of text chunks processed in a single API call, optimizing for performance and API limits.
- Robust Error Handling: 
  - Includes retry mechanisms for handling API errors, with user-defined retry counts.
- Continuous Processing Mode: 
  - Supports streaming input through continuous processing, ideal for real-time data feeds.
- User-Friendly Output: 
  - Color-coded output for easy visual interpretation of severity scores.

## ⬇️ Requirements

### To build from source:
- The Rust programming language [Rust](https://www.rust-lang.org/tools/install)

### To run:
- OpenAI API Key [OpenAI](https://platform.openai.com/account/api-keys)

#### You can export the API key as an environment variable in the shell:
```bash
export OPENAI_API_KEY="YOUR_API_KEY"
```


## 💾 Installation

### 📜 From source:

#### Clone the repository
```bash
git clone https://github.com/ZmoleCristian/VaiLENCE
cd VaiLENCE 
```

#### Build the project
```bash 
cargo build --release
```

### 📦 Using Cargo

```bash
cargo install vailence
```


## ➡️ Usage

VaiLENCE can be executed with various command-line options to suit different use cases:
```
vailence [OPTIONS]
```

### 🚩 Options

```
-s, --severity-min <SEVERITY>  
```
  - Set the minimum severity score for displaying results. The default value is  `0.01`. Adjust this value to increase sensitivity.
```
-i, --input <FILE>
```
  - Specify the path to the input file containing text data. If omitted, VaiLENCE will read from standard input.
```
-o, --output <FILE> 
```
  - Specify the path to the output JSON file. If omitted, results will be printed in the console.
```
-v, --verbose 
```
  - Enable verbose mode to print results to the console, even when an output file is specified. This is useful for debugging or immediate feedback.
```
-c, --chunk-size <CHUNK_SIZE> 
```
  - Define the number of lines to process in each API call. The default is  `100`. Adjust this for performance tuning based on your data size and API limits.
```
-e, --error-retry <RETRY>
```
  - Set the number of times to retry the API call in the event of an error. Default is `3`.
```
-l, --loop 
```
  - Enable continuous processing mode. VaiLENCE will keep running and process input from standard input indefinitely. This is particularly useful for real-time applications.
```  
-h, --help 
  - Display the help message with usage details.
```

### 👉 Examples

#### Basic File Processing

To process a file named  input.txt  and write the output to  output.json , while setting a severity threshold of  `0.05`:
```bash
vailence -i input.txt -o output.json -s 0.05
```

#### Continuous Mode

To run VaiLENCE in continuous mode, processing input as it arrives from standard input:
```bash 
tail -f some_log_file.txt | vailence -l
```

This command will continuously monitor  some_log_file.txt  and process new lines as they are appended.

#### Verbose Mode with Error Retry

To process a file with verbose output and increased error retries:
```bash
vailence -i input.txt -v -e 5
```

This setup will print results directly to the console and retry up to 5 times on errors.

#### Using a Custom Chunk Size

To process large files with a chunk size of 200 lines:
```bash
vailence -i large_file.txt -c 200
```

This configuration helps manage API limits by adjusting the number of lines processed per API call.

## 🥷 Author

- Developed by [Zmole Cristian](https://github.com/ZmoleCristian)

## ⚖️ License

This project is licensed under the BSD 3-Clause License - see the [LICENSE](LICENSE) file for details.


