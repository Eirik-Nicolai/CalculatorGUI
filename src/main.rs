use fltk::{
    app, 
    button::Button, 
    frame::Frame, 
    prelude::*, 
    window::Window, 
    enums::{Color, Key, Shortcut},
    group::{Pack, PackType}
};
use fltk_flex::{Flex, FlexType};
use std::ops::{Deref};


#[derive(Debug, Copy, Clone)]
enum Operator {
    Multiply,
    Divide,
    Plus,
    Minus,
    None
}

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

#[derive(Debug, Copy, Clone)]
enum Message {
    Number(i32),
    Op(Operator),
    Equal(char)
}

struct CalcButton {
    btn: Button,
}

impl CalcButton {
    pub fn new(title: &'static str) -> CalcButton {
        let mut b = CalcButton {
            btn: Button::new(0,0,90,0, title),
        };
        b.btn.set_label_size(20);
        
        match title {
            "=" => {
                b.btn.set_size(180,0);
                b.btn.set_color(Color::Cyan.lighter());
            }
            "*" | "/" | "+" | "-" => {
                b.btn.set_color(Color::Cyan.lighter());
            }
            _ => { 
                b.btn.set_color(Color::Blue.inactive());
            }
        }

        b
    }
}

fn str_from_operator(op:&Operator) -> &str {
    match op {
        Operator::Plus => " + ",
        Operator::Minus => " - ",
        Operator::Multiply => " * ",
        Operator::Divide => " / ",
        _ => panic!("Something went wrong")
    }
}


fn main() {
    //init calculator variables
    let mut out_string = String::from("");
    let mut operator = Operator::None;
    let mut left_calc = String::new();
    let mut right_calc = String::new();
    let mut left_calc_num = 0f64;
    let mut right_calc_num = 0f64;


    //init app and window
    let win_w = 400;
    let win_h = 500;
    let border = 20;
    let but_row = 180;
    let app = app::App::default();

    let frame_h = 100;

    let pack_w_offs = 40;
    let packh_h = 60;

    let mut win = Window::default()
        .with_size(win_w, win_h)
        .with_label("Calculator Example")
        .center_screen();
    win.set_color(Color::Light1);

    //build calulator GUI
    let mut vec_btn_num:Vec<CalcButton> = Vec::new();
    let mut vec_btn_ops:Vec<CalcButton> = Vec::new();

    let mut vpack = Pack::new(border, border, win_w - pack_w_offs, 300,"");
    let mut frame_inp = Frame::new(0,0,win_w-border,frame_h,"Please enter your calculation:");
    
    let mut space_pack = Pack::new(0, 0, win_w - pack_w_offs, 50,"");
    space_pack.end();
    space_pack.set_type(PackType::Horizontal);

    let mut hpack = Pack::new(0,0,win_w - pack_w_offs, packh_h, "");
    vec_btn_num.push(CalcButton::new("1"));
    vec_btn_num.push(CalcButton::new("2"));
    vec_btn_num.push(CalcButton::new("3"));
    vec_btn_ops.push(CalcButton::new("+"));
    hpack.end();
    hpack.set_type(PackType::Horizontal);


    let mut hpack = Pack::new(0,0,win_w - pack_w_offs, packh_h, "");
    vec_btn_num.push(CalcButton::new("4"));
    vec_btn_num.push(CalcButton::new("5"));
    vec_btn_num.push(CalcButton::new("6"));
    vec_btn_ops.push(CalcButton::new("-"));
    hpack.end();
    hpack.set_type(PackType::Horizontal);

    
    let mut hpack = Pack::new(0,0,win_w - pack_w_offs, packh_h, "");
    vec_btn_num.push(CalcButton::new("7"));
    vec_btn_num.push(CalcButton::new("8"));
    vec_btn_num.push(CalcButton::new("9"));
    vec_btn_ops.push(CalcButton::new("*"));
    hpack.end();
    hpack.set_type(PackType::Horizontal);

    
    let mut hpack = Pack::new(0,0,win_w - pack_w_offs, packh_h, "");
    let mut btn_eq = CalcButton::new("=");
    vec_btn_num.push(CalcButton::new("0"));
    vec_btn_ops.push(CalcButton::new("/"));
    hpack.end();
    hpack.set_type(PackType::Horizontal);

    vpack.end();
    win.end();
    win.show();

    let (s, r) = app::channel::<Message>();

    for btn in vec_btn_num.iter_mut() {
        btn.btn.emit(s, Message::Number(btn.btn.label().parse().unwrap()));
    }

    for btn in vec_btn_ops.iter_mut() {
        let label = btn.btn.label();
        let op = match label.as_str() {
            "+" => Operator::Plus,
            "-" => Operator::Minus,
            "*" => Operator::Multiply,
            "/" => Operator::Divide,
            _ => Operator::None
        };
        btn.btn.emit(s, Message::Op(op));
    }

    btn_eq.btn.emit(s, Message::Equal("=".chars().next().unwrap()));

    while app.wait() {
        if let Some(val) = r.recv() {
            match val {
                Message::Number(num) => {
                    if matches!(operator, Operator::None) {
                        out_string.clear();
                        left_calc.push_str(num.to_string().as_str());
                        left_calc_num = left_calc.as_str().parse::<f64>().unwrap();
                        out_string.push_str(left_calc.to_string().as_str());
                    } else {
                        right_calc.push_str(num.to_string().as_str());
                        right_calc_num = right_calc.as_str().parse::<f64>().unwrap();
                        out_string.clear();
                        out_string.push_str(left_calc.to_string().as_str());
                        out_string.push_str(str_from_operator(&operator));
                        out_string.push_str(right_calc.to_string().as_str());
                    }
                },
                Message::Equal(_char) => {
                    if matches!(operator, Operator::None) {
                        println!("Pleas eselect an operator first");
                        continue;
                    }

                    //calculate and clear
                    print!("Left {}, right {}", left_calc, right_calc);
                    let output: f64;
                    match operator {
                        Operator::Plus => output = left_calc_num + right_calc_num,
                        Operator::Minus => output = left_calc_num - right_calc_num,
                        Operator::Multiply => output = left_calc_num * right_calc_num,
                        Operator::Divide => output = left_calc_num / right_calc_num,

                        _ => panic!("Something went very wrong")
                    }
                    out_string.clear();
                    out_string = String::from("Result: ");
                    out_string.push_str(output.to_string().as_str());

                    left_calc.clear();
                    left_calc_num = 0f64;
                    right_calc.clear();
                    right_calc_num = 0f64;

                    operator = Operator::None;
                }
                Message::Op(op) => {
                    operator = op;
                    if matches!(operator, Operator::None) {
                        out_string.push_str(str_from_operator(&operator));
                    } else {
                        out_string.clear();
                        out_string.push_str(left_calc.to_string().as_str());
                        out_string.push_str(str_from_operator(&operator));
                        out_string.push_str(right_calc.to_string().as_str());
                    }
                },
                _ => panic!("Something happened")
            }
        }
        frame_inp.set_label(out_string.as_str());
    }
    
    app.run().unwrap();
}