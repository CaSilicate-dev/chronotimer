use gtk::pango;
use gtk::prelude::{BuilderExtManual};
use gtk::traits::{LabelExt, WidgetExt};
use gtk::{Builder, Label, Window};
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Write;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use chrono::{Utc};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
struct ConfigFile {
    target: String,
    interval: i32,
    precision: i32,

    header: String,
    footer: String,

    header_fontsize: i32,
    time_fontsize: i32,
    footer_fontsize:i32,
}
impl Default for ConfigFile {
    fn default() -> Self {
        ConfigFile {
            target: "2025-11-22 00:00:00".to_string(),
            interval: 100,
            precision: 5,
            header: "Header".to_string(),
            footer: "Footer".to_string(),
            header_fontsize: 50,
            time_fontsize: 50,
            footer_fontsize: 50,
        }
    }
}

fn change_fontsize(label: &Label, fontsize: i32){
    let attr_list = pango::AttrList::new();
    let fontdesc = pango::FontDescription::from_string(format!("Sans {}", fontsize.to_string()).as_str());
    attr_list.insert(pango::AttrFontDesc::new(&fontdesc));

    label.set_attributes(Some(&attr_list));
}

fn main() {
    let file_content = match std::fs::read_to_string("config.yaml") {
        Ok(a) => a,
        Err(e) => {
            eprintln!("Failed to read config file: {}", e);
            let config_init = ConfigFile::default();
            let config_str = serde_yaml::to_string(&config_init).unwrap();
            let mut file = match File::create("config.yaml") {
                Ok(a) => a,
                Err(e) => {
                    eprintln!("Failed to create config file: {}", e);
                    std::process::exit(1);
                }
            };
            match file.write_all(config_str.as_bytes()) {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("Failed to write config file: {}", e);
                    std::process::exit(1);
                }
            };
            match fs::read_to_string("config.yaml") {
                Ok(a) => a,
                Err(e) => {
                    eprintln!("Failed to read config file: {}", e);
                    std::process::exit(1);
                }
            }
        }
    };
    let config: ConfigFile = match serde_yaml::from_str(file_content.as_str()) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("Failed to parse config file: {}", e);
            std::process::exit(1);
        }
    };

    let target = config.target;
    let interval = config.interval;
    let precision = config.precision;

    let header=  config.header;
    let footer = config.footer;

    let header_fontsize = config.header_fontsize;
    let time_fontsize = config.time_fontsize;
    let footer_fontsize = config.footer_fontsize;

    gtk::init().unwrap();

    let glade_src = include_str!("../ui/main.glade");
    let builder = Builder::new();
    builder.add_from_string(glade_src).unwrap();
    let main_window: Window = builder.object("main_window").unwrap();
    let label1: Label = builder.object("l1").unwrap();
    let label2: Label = builder.object("l2").unwrap();
    let label3: Label = builder.object("l3").unwrap();

    let exit_flag = Arc::new(AtomicBool::new(false));
    let exit_flag_clone = exit_flag.clone();
    let thread_exit_flag_clone = exit_flag.clone();

    let label2_clone = label2.clone();

    let (sender, receiver) = std::sync::mpsc::channel::<String>();
    glib::timeout_add_local(Duration::from_millis((interval as f64 * 0.8) as u64),move || match receiver.try_recv() {
        Ok(a) => {
            label2_clone.set_text(a.as_str());
            glib::ControlFlow::Continue
        }
        Err(mpsc::TryRecvError::Empty) => {
            glib::ControlFlow::Continue
        },
        Err(mpsc::TryRecvError::Disconnected) => glib::ControlFlow::Break,
    });

    label1.set_text(header.as_str());
    label3.set_text(footer.as_str());
    
    change_fontsize(&label1, header_fontsize);
    change_fontsize(&label2, time_fontsize);
    change_fontsize(&label3, footer_fontsize);

    thread::spawn(move || {
        let target_clone = target.clone();
        let looptimer_start = Utc::now().timestamp_millis();
        let mut repeat_times = 0;
        loop {
            let target_clone2 = target_clone.clone();
            let looptimer_current = Utc::now().timestamp_millis();
            if looptimer_current - (looptimer_start + ((repeat_times * interval) as i64)) >= 0 {
                repeat_times += 1;
                let target_timestamp = match utils::convert_timestamp(target_clone2) {
                    Ok(a) => a,
                    Err(e) => {
                        eprintln!("Failed to parse target time: {}", e);
                        std::process::exit(1);
                    }
                };
                let current_timestamp = Utc::now().timestamp_millis();
                let delta = ((target_timestamp - current_timestamp) as f64) / 86400000_f64;
                let rounded_delta = utils::advanced_round(delta, precision).to_string();
                sender.send(rounded_delta).unwrap();
            }
            if thread_exit_flag_clone.load(Ordering::Relaxed) {
                break ;
            }
            thread::sleep(Duration::from_millis((interval as f64 * 0.8) as u64));
        }
    });

    main_window.connect_destroy(move |_| {
        exit_flag_clone.store(true, Ordering::Relaxed);
        gtk::main_quit()
    });


    main_window.show_all();
    gtk::main();
}
