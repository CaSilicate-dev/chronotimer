// Copyright (C) 2024 CaSilicate
// SPDX-License-Identifier: GPL-3.0-or-later

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod clockmode;

use chrono::Utc;
use gtk::prelude::BuilderExtManual;
use gtk::traits::{GtkWindowExt, LabelExt, WidgetExt};
use gtk::{Builder, Label, Window};
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Write;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize)]
struct ConfigFile {
    target: String,
    interval: i32,
    precision: i32,
    header: String,
    footer: String,
    header_fontsize: i32,
    time_fontsize: i32,
    footer_fontsize: i32,
    window_title: String,
    window_width: i32,
    window_height: i32,
    unit: String,

    clockmode_settings: ClockmodeConfigConfigfile,
}
#[derive(Debug, Serialize, Deserialize)]
struct ClockmodeConfigConfigfile {
    enable: bool,
    fullscreen: bool,
    showsecond: bool,
    fontsize: i32,
}
impl Default for ClockmodeConfigConfigfile {
    fn default() -> Self {
        Self {
            enable: false,
            fullscreen: false,
            showsecond: true,
            fontsize: 100,
        }
    }
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
            window_title: "ChronoTimer".to_string(),
            window_width: 200,
            window_height: 250,
            unit: "d".to_string(),
            clockmode_settings: ClockmodeConfigConfigfile::default(),
        }
    }
}
fn main() {
    let file_content = std::fs::read_to_string("config.yaml").unwrap_or_else(|e| {
        eprintln!("Failed to read config file: {}", e);
        let config_init = ConfigFile::default();
        let config_str = serde_yaml::to_string(&config_init).unwrap();
        let mut file = File::create("config.yaml").unwrap_or_else(|e| {
            eprintln!("Failed to create config file: {}", e);
            std::process::exit(1);
        });
        file.write_all(config_str.as_bytes()).unwrap_or_else(|e| {
            eprintln!("Failed to write config file: {}", e);
            std::process::exit(1);
        });
        fs::read_to_string("config.yaml").unwrap_or_else(|e| {
            eprintln!("Failed to read config file: {}", e);
            std::process::exit(1);
        })
    });

    let config: ConfigFile = serde_yaml::from_str(file_content.as_str()).unwrap_or_else(|e| {
        eprintln!("Failed to parse config file: {}", e);
        std::process::exit(1);
    });

    let target = config.target;
    let interval = config.interval;
    let precision = config.precision;

    let header = config.header;
    let footer = config.footer;

    let header_fontsize = config.header_fontsize;
    let time_fontsize = config.time_fontsize;
    let footer_fontsize = config.footer_fontsize;

    let window_title = config.window_title;
    let window_width = config.window_width;
    let window_height = config.window_height;
    let unit = config.unit;

    let clockmode = config.clockmode_settings;
    let fullscreen = clockmode.fullscreen;
    let enable_clockmode = clockmode.enable;
    let show_second = clockmode.showsecond;
    let clock_fontsize = clockmode.fontsize;

    if enable_clockmode == false {
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
        let exit_flag_clone2 = exit_flag.clone();
        let thread_exit_flag_clone = exit_flag.clone();

        let label2_clone = label2.clone();

        let (sender, receiver) = std::sync::mpsc::channel::<String>();
        glib::timeout_add_local(
            Duration::from_millis((interval as f64 * 0.8) as u64),
            move || match receiver.try_recv() {
                Ok(a) => {
                    label2_clone.set_text(a.as_str());
                    glib::ControlFlow::Continue
                }
                Err(mpsc::TryRecvError::Empty) => glib::ControlFlow::Continue,
                Err(mpsc::TryRecvError::Disconnected) => glib::ControlFlow::Break,
            },
        );

        label1.set_text(header.as_str());
        label3.set_text(footer.as_str());
        main_window.set_title(window_title.as_str());
        main_window.set_size_request(window_width, window_height);
        clockmode::change_fontsize(&label1, header_fontsize);
        clockmode::change_fontsize(&label2, time_fontsize);
        clockmode::change_fontsize(&label3, footer_fontsize);

        thread::spawn(move || {
            let target_clone = target.clone();
            let looptimer_start = Utc::now().timestamp_millis();
            let mut repeat_times = 0;
            loop {
                let target_clone2 = target_clone.clone();
                let looptimer_current = Utc::now().timestamp_millis();
                if looptimer_current - (looptimer_start + ((repeat_times * interval) as i64)) >= 0 {
                    repeat_times += 1;
                    let target_timestamp =
                        utils::convert_timestamp(target_clone2).unwrap_or_else(|e| {
                            sender
                                .send(format!("Failed to parse target time: {}", e))
                                .unwrap();
                            exit_flag_clone.store(true, Ordering::Relaxed);
                            -1
                        });
                    let current_timestamp = Utc::now().timestamp_millis();
                    let delta = (target_timestamp - current_timestamp) as f64;
                    let remaining =
                        utils::convert_time_unit(delta, unit.as_str()).unwrap_or_else(|e| {
                            sender.send(format!("{}", e)).unwrap();
                            exit_flag_clone.store(true, std::sync::atomic::Ordering::Relaxed);
                            -1.0
                        });
                    let rounded_remaining = utils::advanced_round(remaining, precision);
                    let formated_delta = utils::format_zeros(rounded_remaining, precision);
                    if !thread_exit_flag_clone.load(std::sync::atomic::Ordering::Relaxed) {
                        sender.send(formated_delta).unwrap();
                    }
                }
                if thread_exit_flag_clone.load(Ordering::Relaxed) {
                    break;
                }
                thread::sleep(Duration::from_millis((interval as f64 * 0.8) as u64));
            }
        });

        main_window.connect_destroy(move |_| {
            exit_flag_clone2.store(true, Ordering::Relaxed);
            gtk::main_quit()
        });

        main_window.set_resizable(false);

        main_window.show_all();
        gtk::main();
    } else {
        let c = clockmode::ClockmodeConfig {
            fullscreen,
            winwidth: window_width,
            winhet: window_height,
            interval,
            show_second,
            font_size: clock_fontsize,
        };
        clockmode::clockmode_main(c);
    }
}
