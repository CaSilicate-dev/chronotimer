use gtk::prelude::*;
use gtk::{Builder, Window, FileChooserButton, Button, SpinButton, Entry, RadioButton, Label};
use gtk::prelude::BuilderExtManual;
use serde;
use serde::{Deserialize, Serialize};
use serde_yaml;
use chrono::{Datelike, NaiveDateTime, Timelike};
use gtk::gdk::keys::constants::q;
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
                builder.object("msrd").unwrap(), builder.object("srd").unwrap(), builder.object("mrd").unwrap(),
                builder.object("hrd").unwrap(), builder.object("drd").unwrap(), builder.object("wrd").unwrap(),
                builder.object("mord").unwrap(), builder.object("yrd").unwrap(),
            ],
            statusi: builder.object("statusi").unwrap(),
        }
    }
    fn get_selected_radio(&self) -> Option<RadioButton> {
        self.radios.iter().find(|r| r.is_active()).cloned()
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
    let langconf_c1 = langconf.clone();

    let mainwin = MainWindow::new(&builder);
    let mainwin_c1 = mainwin.clone();
    let mainwin_c2 = mainwin.clone();
    let mainwin_c3 = mainwin.clone();
    let mainwin_c4 = mainwin.clone();
    let mainwin_c5 = mainwin.clone();
    let mainwin_c6 = mainwin.clone();
    let mainwin_c7 = mainwin.clone();
    let mainwin_c8 = mainwin.clone();
    let mainwin_c9 = mainwin.clone();
    let mainwin_c10 = mainwin.clone();
    let mainwin_c11 = mainwin.clone();
    let mainwin_c12 = mainwin.clone();
    let mainwin_c13 = mainwin.clone();
    let mainwin_c14 = mainwin.clone();
    mainwin.openf.connect_clicked(move |_| {
        let filec = mainwin.filec.clone();
        let statusi = mainwin.statusi.clone();
        let file = match filec.filename() {
            Some(a) => a,
            None => {
                statusi.set_text(langconf.no_such_file.as_str());
                return;
            },
        };
        let filename = file.to_string_lossy().into_owned();
        let filecontent = std::fs::read_to_string(filename).unwrap();
        let config: ConfigFile = serde_yaml::from_str(filecontent.as_str()).unwrap();
        let splitedtime = match utils::SplitedTime::from_string(config.target) {
            Ok(a) => a,
            Err(_) => {
                statusi.set_text(langconf.failed_to_parse_time.as_str());
                return;
            }
        };
        mainwin.y.clone().set_value(splitedtime.year as f64);
        mainwin.mo.clone().set_value(splitedtime.month as f64);
        mainwin.d.clone().set_value(splitedtime.day as f64);
        mainwin.h.clone().set_value(splitedtime.hour as f64);
        mainwin.m.clone().set_value(splitedtime.minute as f64);
        mainwin.s.clone().set_value(splitedtime.second as f64);
        mainwin.code.clone().set_text(utils::SplitedTime::to_string(splitedtime).as_str());
        mainwin.itvl.clone().set_value(config.interval as f64);
        mainwin.prec.clone().set_value(config.precision as f64);
        mainwin.header.clone().set_text(config.header.as_str());
        mainwin.footer.clone().set_text(config.footer.as_str());
        mainwin.hfs.clone().set_value(config.header_fontsize as f64);
        mainwin.tfs.clone().set_value(config.time_fontsize as f64);
        mainwin.ffs.clone().set_value(config.footer_fontsize as f64);
        mainwin.wintitle.clone().set_text(config.window_title.as_str());
        mainwin.winwidth.clone().set_value(config.window_width as f64);
        mainwin.winhet.clone().set_value(config.window_height as f64);

        let timeunit = config.unit;
        if timeunit == "ms" {
            mainwin.radios[0].set_active(true);
        } else if timeunit == "s" {
            mainwin.radios[1].set_active(true);
        } else if timeunit == "m" {
            mainwin.radios[2].set_active(true);
        } else if timeunit == "h" {
            mainwin.radios[3].set_active(true);
        } else if timeunit == "d" {
            mainwin.radios[4].set_active(true);
        } else if timeunit == "w" {
            mainwin.radios[5].set_active(true);
        } else if timeunit == "mo" {
            mainwin.radios[6].set_active(true);
        } else if timeunit == "y" {
            mainwin.radios[7].set_active(true);
        } else {
            statusi.set_text(langconf.invalid_time_unit.as_str());
            return;
        }
    });

    mainwin.main_window.connect_delete_event(move |_, _| {
        gtk::main_quit();
        std::process::exit(0);
    });

    mainwin_c1.y.connect_changed(move |_| {
        let splitedtime = SplitedTime {
            year: mainwin_c2.y.value() as i32,
            month: mainwin_c2.mo.value() as i32,
            day: mainwin_c2.d.value() as i32,
            hour: mainwin_c2.h.value() as i32,
            minute: mainwin_c2.m.value() as i32,
            second: mainwin_c2.s.value() as i32,
        };
        let code = SplitedTime::to_string(splitedtime);
        mainwin_c2.code.set_text(code.as_str());
    });

    mainwin_c3.mo.connect_changed(move |_| {
        let splitedtime = SplitedTime {
            year: mainwin_c4.y.value() as i32,
            month: mainwin_c4.mo.value() as i32,
            day: mainwin_c4.d.value() as i32,
            hour: mainwin_c4.h.value() as i32,
            minute: mainwin_c4.m.value() as i32,
            second: mainwin_c4.s.value() as i32,
        };
        let code = SplitedTime::to_string(splitedtime);
        mainwin_c4.code.set_text(code.as_str());
    });

    mainwin_c5.d.connect_changed(move |_| {
        let splitedtime = SplitedTime {
            year: mainwin_c6.y.value() as i32,
            month: mainwin_c6.mo.value() as i32,
            day: mainwin_c6.d.value() as i32,
            hour: mainwin_c6.h.value() as i32,
            minute: mainwin_c6.m.value() as i32,
            second: mainwin_c6.s.value() as i32,
        };
        let code = SplitedTime::to_string(splitedtime);
        mainwin_c6.code.set_text(code.as_str());
    });

    mainwin_c7.h.connect_changed(move |_| {
        let splitedtime = SplitedTime {
            year: mainwin_c8.y.value() as i32,
            month: mainwin_c8.mo.value() as i32,
            day: mainwin_c8.d.value() as i32,
            hour: mainwin_c8.h.value() as i32,
            minute: mainwin_c8.m.value() as i32,
            second: mainwin_c8.s.value() as i32,
        };
        let code = SplitedTime::to_string(splitedtime);
        mainwin_c8.code.set_text(code.as_str());
    });

    mainwin_c9.m.connect_changed(move |_| {
        let splitedtime = SplitedTime {
            year: mainwin_c10.y.value() as i32,
            month: mainwin_c10.mo.value() as i32,
            day: mainwin_c10.d.value() as i32,
            hour: mainwin_c10.h.value() as i32,
            minute: mainwin_c10.m.value() as i32,
            second: mainwin_c10.s.value() as i32,
        };
        let code = SplitedTime::to_string(splitedtime);
        mainwin_c10.code.set_text(code.as_str());
    });

    mainwin_c11.s.connect_changed(move |_| {
        let splitedtime = SplitedTime {
            year: mainwin_c12.y.value() as i32,
            month: mainwin_c12.mo.value() as i32,
            day: mainwin_c12.d.value() as i32,
            hour: mainwin_c12.h.value() as i32,
            minute: mainwin_c12.m.value() as i32,
            second: mainwin_c12.s.value() as i32,
        };
        let code = SplitedTime::to_string(splitedtime);
        mainwin_c12.code.set_text(code.as_str());
    });

    mainwin_c13.savef.connect_clicked(move |_| {
        let splitedtime = SplitedTime {
            year: mainwin_c14.y.value() as i32,
            month: mainwin_c14.mo.value() as i32,
            day: mainwin_c14.d.value() as i32,
            hour: mainwin_c14.h.value() as i32,
            minute: mainwin_c14.m.value() as i32,
            second: mainwin_c14.s.value() as i32,
        };
        let target = SplitedTime::to_string(splitedtime);
        let interval = mainwin_c14.itvl.value() as i32;
        let precision = mainwin_c14.prec.value() as i32;
        let header = mainwin_c14.header.text().to_string();
        let footer = mainwin_c14.footer.text().to_string();
        let hfs = mainwin_c14.hfs.value() as i32;
        let tfs = mainwin_c14.tfs.value() as i32;
        let ffs = mainwin_c14.ffs.value() as i32;
        let wintitle = mainwin_c14.wintitle.text().to_string();
        let winwidth = mainwin_c14.winwidth.value() as i32;
        let winhet = mainwin_c14.winhet.value() as i32;
        let unitradios = mainwin_c14.get_selected_radio().unwrap().buildable_name().unwrap();
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
        let confile_text = serde_yaml::to_string(&configfile).unwrap();
        let filec = mainwin_c14.filec.clone();
        let statusi = mainwin_c14.statusi.clone();
        let file = match filec.filename() {
            Some(a) => a,
            None => {
                statusi.set_text(langconf_c1.no_such_file.as_str());
                return;
            },
        };
        let filename = file.to_string_lossy().into_owned();
        std::fs::write(file, confile_text).unwrap();
    });
    mainwin.main_window.show_all();
    gtk::main();
}
