// Copyright (C) 2024 CaSilicate
// SPDX-License-Identifier: GPL-3.0-or-later

use chrono::Utc;
use chrono::prelude::*;
use gtk::prelude::*;
use gtk::{Label, Window, pango};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub struct ClockmodeConfig {
    pub fullscreen: bool,
    pub winwidth: i32,
    pub winhet: i32,
    pub interval: i32,
    pub show_second: bool,
    pub font_size: i32,
}

pub fn change_fontsize(label: &Label, fontsize: i32) {
    let attr_list = pango::AttrList::new();
    let fontdesc =
        pango::FontDescription::from_string(format!("Sans {}", fontsize.to_string()).as_str());
    attr_list.insert(pango::AttrFontDesc::new(&fontdesc));

    label.set_attributes(Some(&attr_list));
}

pub fn clockmode_main(cmconfig: ClockmodeConfig) {
    let fullscreenc = cmconfig.fullscreen;
    let winwidthc = cmconfig.winwidth;
    let winhetc = cmconfig.winhet;
    let intervalc = cmconfig.interval;
    let show_secondc = cmconfig.show_second;
    let fontsizec = cmconfig.font_size;

    gtk::init().unwrap();
    let glade_src = include_str!("../ui/clock.glade");
    let builder = gtk::Builder::from_string(glade_src);

    let mainwindow: Window = builder.object("mainwindow").unwrap();
    let clocklabel: Label = builder.object("clocklabel").unwrap();
    change_fontsize(&clocklabel, fontsizec);
    if fullscreenc {
        mainwindow.fullscreen();
    } else {
        mainwindow.set_height_request(winhetc);
        mainwindow.set_width_request(winwidthc);
    }

    let exit_flag = Arc::new(AtomicBool::new(false));
    let exit_flag_c1 = exit_flag.clone();
    let exit_flag_c2 = exit_flag.clone();
    let (sender, receiver) = std::sync::mpsc::channel::<String>();
    glib::timeout_add_local(
        Duration::from_millis((intervalc as f64 * 0.8) as u64),
        move || match receiver.try_recv() {
            Ok(a) => {
                clocklabel.set_label(a.as_str());
                glib::ControlFlow::Continue
            }
            Err(mpsc::TryRecvError::Empty) => glib::ControlFlow::Continue,
            Err(mpsc::TryRecvError::Disconnected) => glib::ControlFlow::Break,
        },
    );
    thread::spawn(move || {
        let looptimer_start = Utc::now().timestamp_millis();
        let mut repeat_times = 0;
        loop {
            let looptimer_current = Utc::now().timestamp_millis();
            if looptimer_current - (looptimer_start + ((repeat_times * intervalc) as i64)) >= 0 {
                repeat_times += 1;
                let local: DateTime<Local> = Local::now();
                let formatted_time;
                if show_secondc {
                    formatted_time = local.format("%Y-%m-%d %H:%M:%S").to_string();
                } else {
                    formatted_time = local.format("%Y-%m-%d %H:%M").to_string();
                }
                sender.send(formatted_time).unwrap();
            }
            if !exit_flag_c1.load(Ordering::Relaxed) {
                break;
            }
            thread::sleep(Duration::from_millis((intervalc as f64 * 0.8) as u64));
        }
    });

    mainwindow.connect_destroy(move |_| {
        exit_flag_c2.store(true, Ordering::Relaxed);
        gtk::main_quit()
    });
    mainwindow.show_all();
    gtk::main();
}
