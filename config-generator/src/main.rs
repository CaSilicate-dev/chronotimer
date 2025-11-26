#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use gtk::prelude::BuilderExtManual;
use gtk::prelude::*;
use gtk::{Builder, Button, Entry, FileChooserButton, Label, RadioButton, SpinButton, Window};
use serde;
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::rc::Rc;
use utils::SplitedTime;

#[derive(Serialize, Deserialize, Debug, Clone)]
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
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Lang {
    no_such_file: String,
    failed_to_parse_time: String,
    invalid_time_unit: String,
    failed_to_read_file: String,
    failed_to_parse_config: String,
    failed_to_parse_input: String,
    failed_to_write_file: String,
}
#[derive(Clone)]
struct MainWindow {
    main_window: Window,
    y: SpinButton,
    mo: SpinButton,
    d: SpinButton,
    h: SpinButton,
    m: SpinButton,
    s: SpinButton,
    code: Label,
    itvl: SpinButton,
    prec: SpinButton,
    header: Entry,
    footer: Entry,
    hfs: SpinButton,
    tfs: SpinButton,
    ffs: SpinButton,
    wintitle: Entry,
    winwidth: SpinButton,
    winhet: SpinButton,
    filec: FileChooserButton,
    openf: Button,
    savef: Button,
    radios: [RadioButton; 8],
    statusi: Label,
}
impl MainWindow {
    fn new(builder: &Builder) -> Self {
        Self {
            main_window: builder.object("main_window").unwrap(),
            y: builder.object("y").unwrap(),
            mo: builder.object("mo").unwrap(),
            d: builder.object("d").unwrap(),
            h: builder.object("h").unwrap(),
            m: builder.object("m").unwrap(),
            s: builder.object("s").unwrap(),
            code: builder.object("code").unwrap(),
            itvl: builder.object("itvl").unwrap(),
            prec: builder.object("prec").unwrap(),
            header: builder.object("header").unwrap(),
            footer: builder.object("footer").unwrap(),
            hfs: builder.object("hfs").unwrap(),
            tfs: builder.object("tfs").unwrap(),
            ffs: builder.object("ffs").unwrap(),
            wintitle: builder.object("wintitle").unwrap(),
            winwidth: builder.object("winwidth").unwrap(),
            winhet: builder.object("winhet").unwrap(),
            filec: builder.object("filec").unwrap(),
            openf: builder.object("openf").unwrap(),
            savef: builder.object("savef").unwrap(),
            radios: [
                builder.object("msrd").unwrap(),
                builder.object("srd").unwrap(),
                builder.object("mrd").unwrap(),
                builder.object("hrd").unwrap(),
                builder.object("drd").unwrap(),
                builder.object("wrd").unwrap(),
                builder.object("mord").unwrap(),
                builder.object("yrd").unwrap(),
            ],
            statusi: builder.object("statusi").unwrap(),
        }
    }
    fn get_selected_radio(&self) -> Option<RadioButton> {
        self.radios.iter().find(|r| r.is_active()).cloned()
    }
    fn get_splitedtime(&self) -> SplitedTime {
        SplitedTime {
            year: self.y.value() as i32,
            month: self.mo.value() as i32,
            day: self.d.value() as i32,
            hour: self.h.value() as i32,
            minute: self.m.value() as i32,
            second: self.s.value() as i32,
        }
    }
    fn update_timecode(&self) {
        let mainwin = &self;
        let splitedtime = self.get_splitedtime();
        let code = SplitedTime::to_string(splitedtime);
        mainwin.code.set_text(code.as_str());
    }
}
fn main() {
    gtk::init().unwrap();
    println!("Hello, world!");
    let glade_src = include_str!("../ui/main.glade");
    let builder = Builder::new();
    builder.add_from_string(glade_src).unwrap();

    let langconfct = include_str!("../assets/lang.yaml");
    let langconf: Lang = serde_yaml::from_str(langconfct).unwrap();
    let langconf = Rc::new(langconf);

    let mainwin = MainWindow::new(&builder);
    let mainwin = Rc::new(mainwin);

    let mainwin_c1 = mainwin.clone();
    mainwin_c1.openf.connect_clicked({

        let mainwin_clone = mainwin.clone();
        let langconf_clone = langconf.clone();
        move |_| {
            let filec = mainwin_clone.filec.clone();
            let statusi = mainwin_clone.statusi.clone();
            let file = match filec.filename() {
                Some(a) => a,
                None => {
                    statusi.set_text(langconf_clone.no_such_file.as_str());
                    return;
                }
            };
            let filename = file.to_string_lossy().into_owned();
            let filecontent = match std::fs::read_to_string(filename) {
                Ok(a) => a,
                Err(_) => {
                    statusi.set_text(langconf_clone.failed_to_read_file.as_str());
                    return;
                }
            };
            let config: ConfigFile = match serde_yaml::from_str(filecontent.as_str()) {
                Ok(a) => a,
                Err(_) => {
                    statusi.set_text(langconf_clone.failed_to_parse_config.as_str());
                    return;
                }
            };
            let splitedtime = match SplitedTime::from_string(config.target) {
                Ok(a) => a,
                Err(_) => {
                    statusi.set_text(langconf_clone.failed_to_parse_time.as_str());
                    return;
                }
            };
            mainwin_clone.y.clone().set_value(splitedtime.year as f64);
            mainwin_clone.mo.clone().set_value(splitedtime.month as f64);
            mainwin_clone.d.clone().set_value(splitedtime.day as f64);
            mainwin_clone.h.clone().set_value(splitedtime.hour as f64);
            mainwin_clone.m.clone().set_value(splitedtime.minute as f64);
            mainwin_clone.s.clone().set_value(splitedtime.second as f64);
            mainwin_clone
                .code
                .clone()
                .set_text(SplitedTime::to_string(splitedtime).as_str());
            mainwin_clone.itvl.clone().set_value(config.interval as f64);
            mainwin_clone.prec.clone().set_value(config.precision as f64);
            mainwin_clone.header.clone().set_text(config.header.as_str());
            mainwin_clone.footer.clone().set_text(config.footer.as_str());
            mainwin_clone.hfs.clone().set_value(config.header_fontsize as f64);
            mainwin_clone.tfs.clone().set_value(config.time_fontsize as f64);
            mainwin_clone.ffs.clone().set_value(config.footer_fontsize as f64);
            mainwin_clone
                .wintitle
                .clone()
                .set_text(config.window_title.as_str());
            mainwin_clone
                .winwidth
                .clone()
                .set_value(config.window_width as f64);
            mainwin_clone
                .winhet
                .clone()
                .set_value(config.window_height as f64);

            let timeunit = config.unit;
            if timeunit == "ms" {
                mainwin_clone.radios[0].set_active(true);
            } else if timeunit == "s" {
                mainwin_clone.radios[1].set_active(true);
            } else if timeunit == "m" {
                mainwin_clone.radios[2].set_active(true);
            } else if timeunit == "h" {
                mainwin_clone.radios[3].set_active(true);
            } else if timeunit == "d" {
                mainwin_clone.radios[4].set_active(true);
            } else if timeunit == "w" {
                mainwin_clone.radios[5].set_active(true);
            } else if timeunit == "mo" {
                mainwin_clone.radios[6].set_active(true);
            } else if timeunit == "y" {
                mainwin_clone.radios[7].set_active(true);
            } else {
                statusi.set_text(langconf_clone.invalid_time_unit.as_str());
                return;
            }
        }
    });

    mainwin_c1.main_window.connect_delete_event(move |_, _| {
        gtk::main_quit();
        std::process::exit(0);
    });


    mainwin_c1.y.connect_changed({
        let mainwin = mainwin.clone();
        move |_| {
            mainwin.update_timecode();
        }
    });

    mainwin_c1.mo.connect_changed({
        let mainwin = mainwin.clone();
        move |_| {
            mainwin.update_timecode();
        }
    });

    mainwin_c1.d.connect_changed({
                let mainwin = mainwin.clone();
        move |_| {
            mainwin.update_timecode();
        }
    });

    mainwin_c1.h.connect_changed({
                let mainwin = mainwin.clone();
        move |_| {
            mainwin.update_timecode();
        }
    });

    mainwin_c1.m.connect_changed({
                let mainwin = mainwin.clone();
        move |_| {
            mainwin.update_timecode();
        }
    });

    mainwin_c1.s.connect_changed({
                let mainwin = mainwin.clone();
        move |_| {
            mainwin.update_timecode();
        }
    });

    mainwin_c1.savef.connect_clicked({
        let mainwin_clone = mainwin.clone();
        let langconf_clone = langconf.clone();
        move |_| {
            let splitedtime = mainwin_clone.get_splitedtime();
            let target = SplitedTime::to_string(splitedtime);
            let interval = mainwin_clone.itvl.value() as i32;
            let precision = mainwin_clone.prec.value() as i32;
            let header = mainwin_clone.header.text().to_string();
            let footer = mainwin_clone.footer.text().to_string();
            let hfs = mainwin_clone.hfs.value() as i32;
            let tfs = mainwin_clone.tfs.value() as i32;
            let ffs = mainwin_clone.ffs.value() as i32;
            let wintitle = mainwin_clone.wintitle.text().to_string();
            let winwidth = mainwin_clone.winwidth.value() as i32;
            let winhet = mainwin_clone.winhet.value() as i32;
            let statusi = mainwin_clone.statusi.clone();
            let unitradios = mainwin_clone
                .get_selected_radio()
                .unwrap()
                .buildable_name()
                .unwrap();
            let timeunit;
            if unitradios == "msrd" {
                timeunit = "ms";
            } else if unitradios == "srd" {
                timeunit = "s";
            } else if unitradios == "mrd" {
                timeunit = "m";
            } else if unitradios == "hrd" {
                timeunit = "h";
            } else if unitradios == "drd" {
                timeunit = "d";
            } else if unitradios == "wrd" {
                timeunit = "w";
            } else if unitradios == "mord" {
                timeunit = "m";
            } else if unitradios == "yrd" {
                timeunit = "y";
            } else {
                timeunit = "d";
            }
            let configfile = ConfigFile {
                target,
                interval,
                precision,
                header,
                footer,
                header_fontsize: hfs,
                time_fontsize: tfs,
                footer_fontsize: ffs,
                window_title: wintitle,
                window_width: winwidth,
                window_height: winhet,
                unit: timeunit.to_string(),
            };
            let confile_text = match serde_yaml::to_string(&configfile) {
                Ok(a) => a,
                Err(_) => {
                    statusi.set_text(langconf.failed_to_parse_input.as_str());
                    return;
                }
            };
            let filec = mainwin_clone.filec.clone();
            let statusi = mainwin_clone.statusi.clone();
            let file = match filec.filename() {
                Some(a) => a,
                None => {
                    statusi.set_text(langconf_clone.no_such_file.as_str());
                    return;
                }
            };
            let filename = file.to_string_lossy().into_owned();
            match std::fs::write(filename, confile_text) {
                Ok(a) => a,
                Err(_) => {
                    statusi.set_text(langconf.failed_to_write_file.as_str());
                }
            };
        }
    });
    mainwin.main_window.show_all();
    gtk::main();
}
