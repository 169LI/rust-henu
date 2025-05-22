use serde::{Deserialize, Serialize};
use std::fs;
use std::error::Error;
use std::fmt;

//包含完整的错误处理，使用Result类型并妥善处理serde_json和文件操作的错误
// 自定义错误类型
#[derive(Debug)]
enum BookError {
    FileError(std::io::Error),
    SerializationError(serde_json::Error),
}

impl fmt::Display for BookError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BookError::FileError(e) => write!(f, "文件操作错误: {}", e),
            BookError::SerializationError(e) => write!(f, "序列化错误: {}", e),
        }
    }
}

impl Error for BookError {}

//转换错误
impl From<std::io::Error> for BookError {
    fn from(error: std::io::Error) -> Self {
        BookError::FileError(error)
    }
}
impl From<serde_json::Error> for BookError {
    fn from(error: serde_json::Error) -> Self {
        BookError::SerializationError(error)
    }
}

//Author结构体
#[derive(Serialize, Deserialize)]
struct Author {
    name: String,
    country: String,
}

//Book结构体
#[derive(Serialize, Deserialize)]
struct Book {
    id: u32,
    title: String,
    authors: Vec<Author>,
    metadata: Option<Metadata>,
}

//Metadata结构体
#[derive(Serialize, Deserialize)]
struct Metadata {
    publish_year: u32,
    genres: Vec<String>,
}

// 添加新书
fn add_book(mut books: Vec<Book>, new_book: Book) -> Vec<Book> {
    books.push(new_book);
    books
}

// 序列化并保存到文件
fn save_books(books: &Vec<Book>, filename: &str) -> Result<(), BookError> {
    // 将整个Vec<Book>序列化为格式化的JSON字符串并打印
    let json_str = serde_json::to_string_pretty(books)?;
    println!("{}", json_str);
    // 将该JSON字符串保存到文件
    fs::write(filename, json_str)?;
    Ok(())
}

// 从文件读取并反序列化
fn load_books(filename: &str) -> Result<Vec<Book>, BookError> {
    // 从books.json文件读取JSON字符串，并反序列化回Vec<Book>
    let file_content = fs::read_to_string(filename)?;
    let books: Vec<Book> = serde_json::from_str(&file_content)?;
    Ok(books)
}

fn main() {
    // 创建书籍集合
    let books = vec![
        Book {
            id: 1,
            title: String::from("Rust"),
            authors: vec![Author {
                name: String::from("zhangsan"),
                country: String::from("Us"),
            }],
            metadata: Some(Metadata {
                publish_year: 2000,
                genres: vec![String::from("programming"), String::from("rust")],
            }),
        },
        Book {
            id: 2,
            title: String::from("Java"),
            authors: vec![Author {
                name: String::from("Lisi"),
                country: String::from("Buzhi"),
            }],
            metadata: None,
        },
    ];

    // 保存初始书籍集合
    match save_books(&books, "books.json") {
        Ok(_) => println!("初始书籍已保存到文件"),
        Err(e) => println!("保存初始书籍失败: {}", e),
    }

    // 创建并添加新书
    let new_book = Book {
        id: 3,
        title: String::from("Python"),
        authors: vec![Author {
            name: String::from("Wangwu"),
            country: String::from("China"),
        }],
        metadata: Some(Metadata {
            publish_year: 2023,
            genres: vec![String::from("programming"), String::from("python")],
        }),
    };

    // 添加新书并保存
    let books = add_book(books, new_book);
    match save_books(&books, "books.json") {
        Ok(_) => println!("新书已添加并保存到文件"),
        Err(e) => println!("保存新书失败: {}", e),
    }

    // 读取并显示所有书籍
    match load_books("books.json") {
        Ok(books) => {
            println!("\n当前所有书籍：");
            println!("书籍总数：{}", books.len());
            for book in books {
                println!("书名：{}", book.title);
            }
        }
        Err(e) => println!("读取书籍失败: {}", e),
    }
}
