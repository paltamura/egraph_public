#![allow(non_camel_case_types)]
use crate::functor::Functor;
use crate::{wc, ws};
use clap::{Parser, Subcommand};
use prettytable::{color, Attr};
use prettytable::{Cell, Row, Table};
use std::io;

pub fn evaluate() {
    let args = Args::parse();
    let command = args.command.unwrap();
    match command {
        //Web service
        Commands::ws { run, stop } => {
            if run {
                println!("web service starting...");
                ws::run();
            }
            if stop {
                println!("web service is stoped");
                ws::stop();
            }
        }
        //Modelado
        Commands::createmodel {
            modeltype,
            modelname,
        } => {
            let res = wc::build_and_save(modeltype, modelname);
            println!("{:?}", res);
        }

        Commands::getfunctors { modelname } => {
            let functors = wc::get_functors(modelname);
            let mut table = Table::new();
            for item in functors {
                let mut vec = vec![];
                vec.push(item.name);
                vec.push(item.in_dom.0);
                vec.push(item.out_dom.0);
                table.add_row(get_row(vec, color::WHITE, false));
            }
            table.printstd();
        }

        Commands::getallmorphisms {
            modelname,
            functorname,
        } => {
            let opt_functor = get_functor(modelname, functorname);
            if opt_functor.is_some() {
                print_morphisms(opt_functor.unwrap(), std::option::Option::None);
            }
        }

        Commands::getmorphism {
            modelname,
            functorname,
            morphismid,
        } => {
            let opt_functor = get_functor(modelname, functorname);
            if opt_functor.is_some() {
                print_morphisms(opt_functor.unwrap(), Some(morphismid));
            }
        }

        Commands::editmorphism {
            modelname,
            functorname,
            morphismid,
        } => {
            let opt_functor = get_functor(modelname.clone(), functorname.clone());
            if opt_functor.is_some() {
                print_morphisms(opt_functor.clone().unwrap(), Some(morphismid.clone()));
            }
            let out_dims = opt_functor.unwrap().out_dom.1;
            let mut morphism =
                wc::get_morphism(modelname.clone(), functorname.clone(), morphismid.clone());
            let mut index = 0;
            for out_din in out_dims {
                let message = format!("{:?} > ({:?})", out_din.0, out_din.1);
                let res: String = read_input(&message);
                if out_din.1.contains(&res) {
                    morphism.out_point.0[index] = res;
                }
                index += 1;
            }
            let res: String = read_input("approved? [y|n]");
            if res.to_lowercase().eq("y") || res.to_lowercase().eq("yes") {
                morphism.pass_mark = true;
            }
            if res.to_lowercase().eq("n") || res.to_lowercase().eq("no") {
                morphism.pass_mark = false;
            }
            let res: String = read_input("comment?");
            if !res.trim().eq("") {
                morphism.comment = res;
            }
            wc::set_morphism(
                modelname.clone(),
                functorname.clone(),
                morphismid.clone(),
                morphism,
            );
            let opt_functor = get_functor(modelname.clone(), functorname.clone());
            if opt_functor.is_some() {
                print_morphisms(opt_functor.clone().unwrap(), Some(morphismid.clone()));
            }
        }

        Commands::createorder { modelname } => {
            let id = wc::create_order(modelname);
            println!("{:?}", id);
        }

        Commands::executeorder { orderid } => {
            let res = wc::execute_order(orderid);
            println!("{:?}", res);
        }

        Commands::getorderlog { orderid } => {
            let log = wc::get_order_log(orderid);
            let mut table = Table::new();
            for item in log {
                let mut vec = vec![];
                vec.push(item.0);
                vec.extend(item.1 .0);
                table.add_row(get_row(vec, color::WHITE, false));
            }
            table.printstd();
        }
    }
}

fn get_row(texts: Vec<String>, color: color::Color, fill: bool) -> Row {
    let mut cells: Vec<Cell> = vec![];
    for item in texts {
        let mut cell = Cell::new(&item);

        cell = cell.with_style(Attr::Bold);
        // .with_style(Attr::Italic(true))
        // .with_hspan(2),
        if fill {
            cell = cell.with_style(Attr::BackgroundColor(color::BRIGHT_WHITE));
        }
        cell = cell.with_style(Attr::ForegroundColor(color));

        cells.push(cell);
    }
    Row::new(cells)
}

fn read_input<T: std::str::FromStr>(msg: &str) -> T {
    println!("{}", msg);
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    match input.trim().parse::<T>() {
        Ok(value) => value,
        Err(_) => panic!("parse error"),
    }
}

fn get_functor(modelname: String, functorname: String) -> Option<Functor> {
    let functors = wc::get_functors(modelname);
    let mut func: Option<Functor> = Option::None;
    for fun in functors {
        if fun.name.eq(&functorname) {
            func = Some(fun);
        }
    }
    func
}

fn print_morphisms(functor: Functor, morphism_id: Option<String>) {
    let mut table = Table::new();
    let mut indimnames = vec![];
    for dim in functor.in_dom.1 {
        indimnames.push(dim.0);
    }
    let mut outdimnames = vec![];
    for dim in functor.out_dom.1 {
        outdimnames.push(dim.0);
    }
    //header
    let mut vec = vec![];
    vec.push("ID".to_string());
    vec.extend(indimnames);
    vec.extend(outdimnames);
    vec.push("COMMENTS".to_string());
    vec.push("pass_mark".to_string());
    table.add_row(get_row(vec, color::BRIGHT_BLACK, true));

    let all = &morphism_id.is_none();

    let mut id = "".to_string();
    if !all {
        if let Some(val) = morphism_id {
            id = val
        } else {
            id = "".to_string()
        };
    }
    let refid = id;
    //
    let mut morfs = vec![];
    for item in functor.map {
        morfs.push(item.1);
    }
    morfs.sort_by(|a, b| {
        a.case_id
            .parse::<i32>()
            .unwrap()
            .cmp(&b.case_id.parse::<i32>().unwrap())
    });
    for item in morfs {
        if all.clone() || refid.eq(&item.case_id) {
            let col: color::Color;
            if item.clone().pass_mark {
                col = color::GREEN;
            } else {
                col = color::BRIGHT_RED;
            }
            let mut vec = vec![];
            vec.push(item.clone().case_id);
            vec.extend(item.clone().in_point.0);
            for sitem in item.clone().out_point.0 {
                if item.pass_mark {
                    vec.push(sitem);
                } else {
                    vec.push(format!("{}?", sitem));
                }
            }
            vec.push(item.clone().comment);
            if item.pass_mark {
                vec.push("Approved".to_string());
            } else {
                vec.push("Pending".to_string());
            }
            table.add_row(get_row(vec, col, false));
        }
    }
    table.printstd();
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    // #[clap(short, long)]
    // name: String,
    // #[clap(short, long, default_value_t = 1)]
    // count: u8,
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    // Web service /////////////////////////////////
    ws {
        #[clap(short, long)]
        run: bool,
        #[clap(short, long)]
        stop: bool,
    },

    // Modeling /////////////////////////////////
    createmodel {
        #[clap(long)]
        modeltype: String,
        #[clap(long)]
        modelname: String,
    },

    getfunctors {
        #[clap(short, long)]
        modelname: String,
    },

    getallmorphisms {
        #[clap(short, long)]
        modelname: String,
        #[clap(short, long)]
        functorname: String,
    },

    getmorphism {
        #[clap(long)]
        modelname: String,
        #[clap(short, long)]
        functorname: String,
        #[clap(long)]
        morphismid: String,
    },

    editmorphism {
        #[clap(long)]
        modelname: String,
        #[clap(short, long)]
        functorname: String,
        #[clap(long)]
        morphismid: String,
    },

    // Execution /////////////////////////////////
    createorder {
        #[clap(short, long)]
        modelname: String,
    },

    executeorder {
        #[clap(short, long)]
        orderid: String,
    },

    getorderlog {
        #[clap(short, long)]
        orderid: String,
    },
}