use std::{fs, io};

use clap::Parser;

#[derive(Parser)]
struct Args {
    /// 指定目录 默认为当前目录
    #[arg(short, long, default_value = ".")]
    location: String,
    /// 修改文件的后缀 不指定默认为文件夹
    #[arg(short, long)]
    ext: Option<String>,
    /// 修改后文件名的前缀 例如 "我的文件"
    #[arg(short, long)]
    prefix: String,
    /// 默认为预览模式，即打印出替换的变化，如果要实际执行增加参数 -f 1
    #[arg(short, long)]
    flags: Option<u8>,
}

fn main() {
    let args = Args::parse();

    let entries = fs::read_dir(args.location)
        .unwrap()
        .filter(|entry| {
            let path = entry.as_ref().unwrap().path();
            match &args.ext {
                None => path.is_dir(),
                Some(e) => {
                    println!("{:?}", path.extension());
                    path.is_file()
                        && path.extension().is_some()
                        && path.extension().unwrap().to_str().unwrap().eq(e)
                }
            }
        })
        .collect::<Result<Vec<_>, io::Error>>()
        .unwrap();

    // println!("{:?}", entries);

    for i in 0..entries.len() {
        let e = &entries[i];
        let parent = String::from(e.path().parent().unwrap().to_str().unwrap());

        match e.path().is_file() {
            false => {
                println!(
                    "{} -> {}",
                    e.path().to_str().unwrap(),
                    format!("{}/{}{}", parent, args.prefix, i)
                );
                if args.flags.is_some() {
                    fs::rename(e.path(), format!("{}/{}{}", parent, args.prefix, i)).unwrap()
                }
            }
            true => {
                let ext = String::from(e.path().extension().unwrap().to_str().unwrap());
                println!(
                    "{} -> {}",
                    e.path().to_str().unwrap(),
                    format!("{}/{}{}.{}", parent, args.prefix, i, ext)
                );
                if args.flags.is_some() {
                    fs::rename(e.path(), format!("{}/{}{}.{}", parent, args.prefix, i, ext))
                        .unwrap();
                }
            }
        }
    }
}
