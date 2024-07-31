use eframe::{
    egui::{
	Color32,
	self,
	Stroke,
    }
};

use egui_plot::{
    Plot,
    PlotPoints
};

struct Foo { }

const TRIANGLES : &[[(f64,f64);3]] = &[
    [(-5.06499887e-1, -1.75000000e0),
     (-5.59192896e-1, -1.82903767e0),
     (-5.05881429e-1, -1.75000000e0)], // err
];

impl Foo {
    fn new(_cc:&eframe::CreationContext<'_>)->Self {
	Self { }
    }

    fn draw_mesh(&self,ui:&mut egui_plot::PlotUi) {
	for tri in TRIANGLES {
	    self.draw_tri(ui,tri);
	}
    }
    
    fn draw_tri(&self,ui:&mut egui_plot::PlotUi,tri:&[(f64,f64);3]) {
	let pp = PlotPoints::from_iter(tri.iter().map(|p| [p.0,p.1]));
	let color = Color32::YELLOW;
	let fill_color = Color32::RED;
	ui.polygon(egui_plot::Polygon::new(pp)
		   .fill_color(fill_color)
		   .stroke(Stroke::new(1.0,color)));
    }
}

impl eframe::App for Foo {
    fn update(&mut self,ctx:&egui::Context,_frame:&mut eframe::Frame) {
	egui::CentralPanel::default().show(ctx,|ui| {
	    egui::Frame::canvas(ui.style()).show(ui,|ui| {
		let plot = Plot::new("Bar")
		    .auto_bounds([false,false].into())
		    .include_x(-1.0)
		    .include_x(1.0)
		    .include_y(-2.0)
		    .include_y(2.0);
		plot.show(ui,|ui| self.draw_mesh(ui));
	    });
	});
    }
}

fn main()->Result<(), eframe::Error> {
    eframe::run_native(
	"Foo",
	eframe::NativeOptions::default(),
	Box::new(|cc| Ok(Box::new(Foo::new(cc))))
    )
}
