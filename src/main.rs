use eframe::egui;
use std::f64::consts::TAU;

use egui::plot::{GridInput, GridMark};
use egui::*;
use plot::{
    Arrows, Bar, BarChart, BoxElem, BoxPlot, BoxSpread, CoordinatesFormatter, Corner, HLine,
    Legend, Line, LineStyle, MarkerShape, Plot, PlotImage, PlotPoint, PlotPoints, Points, Polygon,
    Text, VLine,
};

#[derive(Default)]
struct MyApp{
    math_expr:String
}

impl MyApp{
    fn new(cc: &eframe::CreationContext<'_>)->Self{
        let sin = (0..1000).map(|i| {
            let x = i as f64 * 0.01;
            (x, x.sin())
        });
        println!("{:?}",sin);
        Self::default() 
    }
}

impl eframe::App for MyApp{
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame){
        egui::CentralPanel::default().show(ctx, |ui| {
            let n = 1000;
            ui.heading("Enter Math Expr");
            ui.text_edit_singleline(&mut self.math_expr);
            ui.label(&self.math_expr);
            let sin_values: Vec<_> = (0..=n)
            .map(|i| remap(i as f64, 0.0..=n as f64, -TAU..=TAU))
            .map(|i| [i, i.sin()])
            .collect();

            let cos_values: Vec<_> = (0..=n)
            .map(|i| remap(i as f64, 0.0..=n as f64, -TAU..=TAU))
            .map(|i| [i, i.cos()])
            .collect();

            let line = Line::new(sin_values);
            let line_cos = Line::new(cos_values);

            let plot = Plot::new("items_demo")
            .legend(Legend::default().position(Corner::RightBottom))
            .show_x(false)
            .show_y(false)
            .data_aspect(1.0);
            plot.show(ui, |plot_ui| {
                plot_ui.line(line.name("Sin(x)"));
                plot_ui.line(line_cos.name("Cos(x)"));
            });

        });
    }
}

fn main(){
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("FFT", native_options, Box::new(|cc| Box::new(MyApp::new(cc))));
}

