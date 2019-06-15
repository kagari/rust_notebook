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

fn run(input: &str) -> String {
    let path = Path::new("./tmp.rs");
    std::fs::write(path, input);
    let err = Command::new("rustc").arg(path).output().expect("failed to compile").stderr;
    if !err.is_empty() {
        return String::from_utf8(err).unwrap();
    };
    // コンパイル時にエラーが出た場合に以下の処理を行わない
    let output = Command::new("./tmp").output().expect("failed to execute process");
    Command::new("rm").args(&["-f", "./tmp.rs", "./tmp"]).output().expect("failed to remove tmp file");
    String::from_utf8(output.stdout).unwrap()
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
