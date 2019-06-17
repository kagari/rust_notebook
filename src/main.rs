#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate json;

use std::path::Path;
use std::fs::File;
use std::io::{Read};
use std::process::Command; // Rust のプロセスを立てる
use rocket::response::content::Html;
use rocket_contrib::json::JsonValue;

fn _sorround_main_func(body: &str) -> String {
    format!("fn main() {{\n{}\n}}", body)
}

fn run(input: &str) -> String {
    let path = Path::new("./tmp.rs");
    let main = _sorround_main_func(input);
    std::fs::write(path, main);
    let output = Command::new("rustc").arg(path).output().expect("failed to compile");
    // コンパイル時にエラーが出た場合に実行処理を行わなわず、エラー文を返す
    if !output.status.success() {
        Command::new("rm").args(&["-f", "./tmp.rs"]).output().expect("failed to remove tmp file");
        return String::from_utf8(output.stderr).unwrap();
    };
    let output = Command::new("./tmp").output().expect("failed to execute child").stdout;
    Command::new("rm").args(&["-f", "./tmp.rs", "./tmp"]).output().expect("failed to remove tmp file");
    String::from_utf8(output).unwrap()
}

#[post("/", data="<input>")]
fn run_post_code(input: String) -> JsonValue {
    // 受け取ったコードを実行する
    let output = run(input.as_str());
    // 結果をjsonに格納する
    json!(
        {
            "name": "stdout",
            "output_type": "stream",
            "text": [
                output
            ]
        })
}

#[get("/tree/<path>")]
fn entrunce(path: String) -> Html<String> {
    println!("{}", path);
    let path = Path::new(&path);
    if path.is_dir() {
        let list_str = _make_dir_list_html(path);
        let html = format!("<html>
<body>
<h1>Welcome to Rust Notebook!</h1>
{}
</body>
</html>", list_str);
        return Html(html)
    }
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    Html(contents)
}

#[get("/tree")]
fn index() -> Html<String> {
    // ルート以下のファイルまたはディレクトリを探す
    let path = Path::new("./");
    let list_str = _make_dir_list_html(path);
    let html = format!("<html>
<body>
<h1>Welcome to Rust Notebook!</h1>
{}
</body>
</html>", list_str);
    Html(html)
}

// カレントディレクトリ内のファイルやディレクトリをhtml形式で返す
fn _make_dir_list_html(path: &Path) -> String {
    let plist = std::fs::read_dir(path);
    let mut list_str = String::new();
    
    if let Ok(list) = plist {
        for name in list {
            match name {
                Ok(x) => {
                    let fname = x.file_name().into_string().unwrap();
                    let flink = format!("<a href=\"/tree/{fname}\"><span>{fname}</span></a><br>\n", fname=fname);
                    list_str.push_str(&flink);
                },
                Err(_) => (),
            }
        }
    }
    list_str
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, entrunce])
        .mount("/api", routes![run_post_code])
        .launch();
}
