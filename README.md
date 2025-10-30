# 🪙 **Curc** - Simple **Cur**rancy **C**onverter

### Description
Currancy Converter written in Rust. Easy to use project based on [this API](https://github.com/fawazahmed0/currency-api). Feel free to fork it and use it in your projects. It's my first project written in Rust, so if you encounter any issues I would appricate if you report them and leave feedback.

### Download
To install **curc** you need to download Cargo (Rust's package manager) first.

MacOS/Linux:
```
curl https://sh.rustup.rs -sSf | sh
```
On Windows [click here](https://doc.rust-lang.org/cargo/getting-started/installation.html) to download.

After you install Cargo on your machine, clone this repo then enter the directory and enter:
```
cargo install --path .
```
If you encounter any issues check [here](https://doc.rust-lang.org/cargo/getting-started/installation.html)

### Usage
It's pretty simple you have two "modes":
+ convert:
```
curc <VALUE> --from <CURRENCY_CODE> --to <CURRENCY_CODE>
```
where VALUE is amount of currency. All CURRENCY_CODEs are avaliable in 2nd mode which is
+ list:
```
curc list
``` 
**Note** that program is installed in two versions: cruc and currency_converter. You can use both names.

### License
This project is licensed under the **GNU General Public License v3.0**

See the [LICENSE](LICENSE)

### Contribute
*in the future*
