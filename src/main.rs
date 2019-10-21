use std::ops::Add;
use std::fs::{ File };
use std::io::{BufReader, BufRead, BufWriter, Write};
use chrono::{Local, NaiveDateTime, Duration};

fn usage() {
    let exe_path = std::env::current_exe().unwrap();
    println!("{:?} [ics file path]", exe_path.file_name().unwrap());
    println!("→ 3か月前からの予定データをicsファイルから切り出し、mod.icsファイルを生成します。", )
}

fn main()-> Result<(), std::io::Error> {
    let ics_path: &str;// = r#"C:\Users\rh\Desktop\20191021_ryuta.hayashi@fujifilm.com.ical\ryuta.hayashi@fujifilm.com.ics"#;
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        ics_path = &args[1];
    }
    else {
        usage();
        return Ok(());
    }

    let f = File::open(ics_path)?;
    let mut reader = BufReader::new(f);
    let mut path = std::path::PathBuf::from(ics_path);
    path.pop();
    path.push("mod.ics");

    let w = File::create(path)?;
    let mut writer = BufWriter::new(w);
    let mut do_write = true;
    let mut tmp_buffer = String::new();
    let mut into_block = false;
    let mut task_date: NaiveDateTime = NaiveDateTime::parse_from_str("1901/01/01 00:00:00", "%Y/%m/%d %H:%M:%S").unwrap();
    let dt_now = Local::now().add(Duration::days(-90));

    println!("基準日: [{}]", dt_now);

    loop {
        let mut line = String::new();
        let len = reader.read_line(&mut line)?;
        if len == 0 {
            break;
        }
        tmp_buffer += &line;

        if line.trim_end() == "BEGIN:VEVENT" {
            into_block = true;
            do_write = false;
        }
        if line.trim_end() == "END:VEVENT" {
            into_block = false;
            // check task_date => reflect do_write
            do_write = task_date.timestamp() > dt_now.timestamp();
        }

        if line.starts_with("CREATED") {
            let mut sp = line.split(":");
            sp.next();
            let tmp_task_date: String = sp.next().unwrap().to_string();
            task_date = NaiveDateTime::parse_from_str(tmp_task_date.trim_end_matches("Z\r\n"), "%Y%m%dT%H%M%S").unwrap();

        }

        if do_write {
            writer.write(tmp_buffer.as_bytes())?;
            tmp_buffer.clear();
        }
        if into_block == false {
            tmp_buffer.clear();
        }
        if line.trim_end() == "END:VTIMEZONE" {
            do_write = false;
        }
    }
    Ok(())
}
