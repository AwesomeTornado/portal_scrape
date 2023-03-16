use std::borrow::ToOwned;
use std::fs;
use std::fs::File;
use reqwest;
use std::io::{Read, Write};
use reqwest::header::USER_AGENT;
use reqwest::header::REFERER;

const mpls:&str = "https://sisportal.mpls.k12.mn.us/image.ashx?name=";
const s241:&str = "https://sisstudentsts-241.campusnet.net/image.ashx?name=";


fn run() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let mut idList = fs::File::create("./database_dump/_idList.txt").unwrap();
    for id in 10000..10020{
        let mut res:reqwest::Response = client
            .get(&(s241.to_string() + &id.to_string() + ".JPG") as &str)
            .header(REFERER, "https://sisportal.mpls.k12.mn.us/gradebook_student_schedule_enroll.aspx?")
            .send()
            .unwrap();
        if(res.status() == 200){
            println!("id# {} exists", id);
            let mut body = Vec::new();
            let mut file = fs::File::create(&("./database_dump/".to_string() + &format!("{:0>8}", "110")id.to_string() + ".html") as &str).unwrap();
            res.read_to_end(&mut body)?;
            file.write_all(body.as_mut_slice());
            idList.write(id.to_string().as_bytes());
            idList.write(b"\n");
        }
    }
    Ok(())
}

fn ver_dir(){
    let dir = fs::read_dir("./database_dump");
    if(dir.is_err()){
        fs::create_dir("./database_dump");
    }else if dir.unwrap().count() > 0 {
        panic!("expected empty directory, directory was not empty!");
    }
}

fn main() {
    ver_dir();
    run();
}
