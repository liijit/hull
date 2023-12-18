use std::{fmt, path};
use std::cell::RefCell;
use std::rc::Rc;
use std::fs::File;
use std::env;
use std::path::PathBuf;
use std::io::Write;

use usvg::filter::{ Primitive, Kind, Input, DropShadow, Merge, ColorInterpolation };
use usvg::{Tree, TreeWriting, XmlOptions, NodeExt, NormalizedF32, Units, PositiveF32, Color};
use usvg::tiny_skia_path::PathBuilder;

#[derive(Default, Debug, Clone, Copy)]
pub struct Size {
	width: f32,
	height: f32
}

#[derive(Default, Debug)]
pub struct Point{
	x: f32,
	y: f32,
	radius: f32
}

// TODO: change this into an enum to utilise match
#[derive(Default, Debug)]
pub struct Points {
	lt: Point,
	rt: Point,
	rb: Point,
	lb: Point
}

#[derive(Debug, Clone)]
pub struct LayerProps {
	path: PathBuilder,
	shrink: f32,
}

impl LayerProps {
	pub fn clear(&mut self) {
		self.path = PathBuilder::new();
		self.shrink = 0.;
	}
}

impl<T> StackGeneric<T>
where
	T: std::fmt::Debug + Clone,
{
	pub fn new() -> Self {
		StackGeneric { layers: Rc::new(RefCell::new(vec![])) }
	}

	pub fn push(&mut self, item: T) {
		// self.layers.push(item)
		self.layers.borrow_mut().push(item)
	}

	pub fn clear(&mut self) {
		// self.layers.clear()
		self.layers.borrow_mut().clear()
	}
}

#[derive(Clone, Debug)]
pub struct StackGeneric<T: std::fmt::Debug> {
	layers: Rc<RefCell<Vec<T>>>,
}

impl<T> Default for StackGeneric<T> where T: std::fmt::Debug {
	fn default() -> Self {
		Self {
		    layers: Rc::new(RefCell::new(vec![]))
		}
	}
}

impl<T> Iterator for StackGeneric<T>
where
	T: std::fmt::Debug + Clone,
{
	type Item = T;

	fn next(&mut self) -> Option<Self::Item> {
		if self.layers.borrow().len() == 0 {
			return None;
		}
		self.layers.borrow_mut().pop()
	}
}

#[derive(Debug, Clone)]
pub enum Layer {
	Stroke(LayerProps),
	Fill(LayerProps),
	Shadow(LayerProps)
}

// `Size` needs to be greater than 0.0
// IsValidLength is implemented for Size, ViewBox, PathBox, Rect

impl Default for Polygon {
    fn default() -> Self {
        Self {
            tree: TreeStruct(usvg::Tree {
                size: usvg::Size::from_wh(1., 1.).unwrap(),
                view_box: usvg::ViewBox {
                    rect: usvg::NonZeroRect::from_xywh(0., 0., 1., 1.).unwrap(),
                    aspect: usvg::AspectRatio::default(),
                },
                root: usvg::Node::new(usvg::NodeKind::Group(usvg::Group {
                	filters: vec![],
                    ..Default::default()
                })),
            }),
            size: Size::default(),
            margin: Margin::default(),
            color: Colors::default(),
            stroke: Stroke::default(),
            points: Points::default(),
            path_data: StackGeneric::new(),
        }
    }
}

// `Tree` with an 'orphan' like wrapper for Debugging
struct TreeStruct(Tree);
impl fmt::Debug for TreeStruct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TreeStruct")
            .field("size", &self.0.size)
            .field("view_box", &self.0.view_box)
            .field("root", &self.0.root)
            .finish()
    }
}

// Polygon hard-coded to 4 sides
#[derive(Debug)]
pub struct Polygon {
	tree: TreeStruct,
	size: Size,
	margin: Margin,
	color: Colors,
	stroke: Stroke,
	points: Points,
	path_data: StackGeneric<Layer>
}

// Margins expand out to the following
// Four  ( left, top, right, down)
// Three ( left, y-axis, right, y-axis)
// Two   ( x-axis, y-axis, x-axis, y-axis)
// One   ( side, side, side, side)

#[derive(Debug, Copy, Clone)]
enum Margin {
	Four  ( f32, f32, f32, f32),
	Three ( f32, f32, f32),
	Two   ( f32, f32),
	One   ( f32)
}

impl Default for Margin {
	fn default() -> Self {
		Self::Four(0., 0., 0., 0.);
		Self::Three(0., 0., 0.);
		Self::Two(0., 0.);
		Self::One(0.)
	}
}

#[derive(Debug)]
enum Split {
	Equal,
	Half,
	Third,
	Quater,
	Fifth,
}

type Spliter = (f32, u8, Split);

#[derive(Debug)]
struct DropShadowMultipler {
	max: Option<f32>,
	multipler_deviation: (f32, f32),
	opacity_expand: Vec<f32>,
	opacity_counter: usize,
	x_deviation: f32,
	y_deviation: f32,
}

impl DropShadowMultipler {
	fn new() -> Self {
		Self {
			max: Some(1.),
			multipler_deviation: (1., 1.),
			opacity_counter: 0,
			opacity_expand: vec![],
			x_deviation: 0.5,
			y_deviation: 0.5,
		}
	}

	fn expand_opacity_multipler(&mut self, vec: Vec<Spliter>, accuracy: usize) {
        // TODO: create different split values that can be one side heavy.

		self.opacity_expand = vec.into_iter().map(|f| {
        	let split =	match f.2 {
        		Split::Equal => {f.0 / f.1 as f32}
        		Split::Half => {f.0}
        		Split::Third => {f.0}
        		Split::Quater => {f.0}
        		Split::Fifth => {f.0}
        	};

        	let mut splits = vec![];
        	for _ in 0..f.1 {
        		splits.push(split);
        	}
        	splits
        }).flatten().collect();

		// needs better implementation rather then indexing directly
		// checks lengths and fills blanks
		if self.opacity_expand.len() < accuracy {
			for _ in self.opacity_expand.len()..accuracy {
				self.opacity_expand.push(0.);
			}
		}
	}
}

impl Iterator for DropShadowMultipler {
	type Item = (f32, f32, f32);
	fn next(&mut self) -> Option<Self::Item> {
	    self.x_deviation += self.x_deviation / 100. * self.max.unwrap() * self.multipler_deviation.0;
	    self.y_deviation += self.y_deviation / 100. * self.max.unwrap() * self.multipler_deviation.1;

        self.opacity_counter += 1;
	    Some((self.x_deviation, self.y_deviation, self.opacity_expand[self.opacity_counter-1]))
	}
}

trait Drawer {
	fn draw(&self, points: &Points) -> PathBuilder;
	fn construct_layer_nodes(&self) {}
}

impl Drawer for Polygon {

	//  TODO: use usvg SkPath re-implementation instead. See: path_builder.rs

	fn draw(&self, points: &Points) -> PathBuilder {
		let mut path: PathBuilder = PathBuilder::new();
		PathBuilder::move_to(&mut path, points.lb.x, points.lb.y - points.lb.radius);

		PathBuilder::line_to(&mut path, points.lb.x, points.lb.y - points.lb.radius);
		PathBuilder::line_to(&mut path, points.lt.x, points.lt.y + points.lt.radius);

		PathBuilder::cubic_to(
			&mut path,
			points.lt.x,
			points.lt.y + points.lt.radius,
			points.lt.x,
			points.lt.y,
			points.lt.x + points.lt.radius,
			points.lt.y,
		);

		PathBuilder::line_to(&mut path, points.lt.x + points.lt.radius, points.lt.y);
		PathBuilder::line_to(&mut path, points.rt.x - points.rt.radius, points.rt.y);

		PathBuilder::cubic_to(
			&mut path,
			points.rt.x - points.rt.radius,
			points.rt.y,
			points.rt.x,
			points.rt.y,
			points.rt.x,
			points.rt.y + points.rt.radius,
		);

		PathBuilder::line_to(&mut path, points.rb.x, points.rt.y + points.rt.radius);
		PathBuilder::line_to(&mut path, points.rb.x, points.rb.y - points.rb.radius);

		PathBuilder::cubic_to(
			&mut path,
			points.rb.x,
			points.rb.y - points.rb.radius,
			points.rb.x,
			points.rb.y,
			points.rb.x - points.rb.radius,
			points.rb.y,
		);

		PathBuilder::line_to(&mut path, points.rb.x - points.rb.radius, points.rb.y);
		PathBuilder::line_to(&mut path, points.lb.x + points.lb.radius, points.lb.y);

		PathBuilder::cubic_to(
			&mut path,
			points.lb.x + points.lb.radius,
			points.lb.y,
			points.lb.x,
			points.lb.y,
			points.lb.x,
			points.lb.y - points.lb.radius,
		);

		PathBuilder::close(&mut path);

		path
	}

	fn construct_layer_nodes(&self) {
		for e in self.path_data.layers.borrow_mut().iter() {
			match e {
				Layer::Fill(props) => {

					let mut path = usvg::Path::new(Rc::new(props.path.clone().finish().unwrap()));
					path.id = "fill".to_string();
					path.fill = Some(usvg::Fill {
                                paint: usvg::Paint::Color(self.color.fill.unwrap()),
                                opacity: NormalizedF32::new(0.8).unwrap(),
                                ..usvg::Fill::default()
                            });

					let node = usvg::NodeKind::Path(path);
					self.tree.0.root.append_kind(node);
				},
				Layer::Stroke(props) => {
					// return if layer hasn't been shrunk, this would escape the
					// bounds of the image
					// return

					if props.shrink == 0. {
						return
					};

					let mut path = usvg::Path::new(Rc::new(props.path.clone().finish().unwrap()));
					path.id = "stroke".to_string();

					path.stroke = Some(usvg::Stroke {
	                    paint: usvg::Paint::Color(self.color.stroke.unwrap()),
	                    width: usvg::NonZeroPositiveF32::new(props.shrink).unwrap(),
                        opacity: NormalizedF32::new(0.7).unwrap(),
	                    // linejoin: usvg::LineJoin::Round,
	                    ..usvg::Stroke::default()
	                });
					let node = usvg::NodeKind::Path(path);
					self.tree.0.root.append_kind(node);
				},
				Layer::Shadow(props) => {

					// calculate canvas -> margin distance, use the smallest
					// value if there are < 2 margins defined

					let mut min: Vec<f32> = vec![];

					match self.margin  {
						Margin::Four(l, t, r, d) => {
							min.push(l);
							min.push(t);
							min.push(r);
							min.push(d);
						}
						Margin::Three(l, y, r) => {
							min.push(l);
							min.push(y);
							min.push(r);
						}
						Margin::Two(x, y) => {
							min.push(x);
							min.push(y);
						}
						Margin::One(m) => {
							min.push(m);
						}
					}

					// returns minimum value that is greater then 0.
					// TODO: add an option which overwrites this

                    let deviation = min.into_iter().filter(|n| n > &0.0).reduce(f32::min).unwrap();

                    let accuracy = 6;

					// We could possibly have different combinations of shadows
					// Layering technique suggested by:
					// https://tobiasahlin.com/blog/layered-smooth-box-shadows/

                    let mut shadow_sharp = DropShadowMultipler::new();
                    shadow_sharp.max = Some(deviation);
                    shadow_sharp.x_deviation = 1.;
                    shadow_sharp.y_deviation = 1.2;
                    shadow_sharp.multipler_deviation = (5.0, 5.0);

                    let splits: Vec<Spliter> = vec![
	                    (0.5, 2, Split::Equal),
	                    (0.125, 2, Split::Equal),
	                    (0.1, 2, Split::Equal)
                    ];
                    shadow_sharp.expand_opacity_multipler(splits, accuracy);

                    let mut merge_input: Vec<Input> = vec![];

                    let drop_shadow_arr: Vec<_> = shadow_sharp.take(accuracy as usize)
	                	.into_iter()
	                	.enumerate()
	                	.map(| (ix, t) | {
	                		let result = ["drop_shadow".to_string(), ix.to_string()].join("_");
	                		merge_input.push(Input::Reference(result.clone()));
			                Primitive {
								kind: Kind::DropShadow(DropShadow {
									input: Input::SourceGraphic,
									dx: 0.,
									dy: 0.,
									std_dev_x: PositiveF32::new(t.0).unwrap(),
									std_dev_y: PositiveF32::new(t.1).unwrap(),
									color: self.color.shadow.unwrap(),
									opacity: NormalizedF32::new(t.2).unwrap(),
								}),
			                    x: None,
			                    y: None,
			                    width: None,
			                    height: None,
			                    color_interpolation: ColorInterpolation::LinearRGB,
			                    result,
			                }
	                	 })
	                	.collect()
	                	;

					// usvg preproccesor will group the same Kinds, we Kind Merge
					// these below with their resulting name

		                let merge = Primitive {
							kind: Kind::Merge(Merge {
								inputs: merge_input
							}),
		                    x: None,
		                    y: None,
		                    width: None,
		                    height: None,
		                    color_interpolation: ColorInterpolation::LinearRGB,
		                    result: "merge".to_string(),
		                };

		            let mut primitives = vec![];
		            primitives.extend(drop_shadow_arr);
		            primitives.push(merge);

					for node in self.tree.0.root.descendants().take(1) {

					        // self.tree.0.root.append_kind(
					        // 	usvg::NodeKind::Group(
							// 	usvg::Group {
							// 	    clip_path: Some(Rc::new(usvg::ClipPath {
							// 	    	id: "example".to_string(),
				            //             units: Units::UserSpaceOnUse,
							// 	    	clip_path: None,
							// 	    	root: node.clone(),
							// 			..Default::default()
							// 	    })),
							// 		id: "chuwbaka".to_string(),
							// 		..Default::default()
							// 		}
							// 	)
				        	// );

				        if let usvg::NodeKind::Group(ref mut g) = *node.borrow_mut() {

							let filter = Rc::new(usvg::filter::Filter {
					                id: "lol23".to_string(),
					                primitive_units: Units::UserSpaceOnUse,
					                // clip_path
					                primitives: primitives.clone(),
					                rect: usvg::NonZeroRect::from_xywh(0., 0., self.size.width, self.size.height).unwrap(),
					                units: Units::UserSpaceOnUse
			            		});

							g.filters.push(filter);

							for item in g.clip_path.iter_mut() {

			            		// *item = Rc::new(usvg::ClipPath {
								//     	id: "screen".to_string(),
				                //         units: Units::UserSpaceOnUse,

								    	// clip_path: Some(Rc::new(usvg::ClipPath {
									    // 	id: "t".to_string(),
					                    //     units: Units::UserSpaceOnUse,
									    // 	clip_path: None,

									    // 	// root: Node::new(test_node.clone()),
									    // 	// root: Node::new(usvg::NodeKind::Group(Group::default())).append_kind(test_node.clone()),

									    // 	// root: Node::new(test_node_group.clone()),
									    // 	root: Node::new(test_node_group.clone()),

										// 	..Default::default()
									    // }),
								    	// root: Node::new(test_node_group.clone()),
								    	// root: Node::new(usvg::Group::default()),
								    	// root: Node::new(usvg::NodeKind::Group(Group::default())).append_kind(test_node.clone()),

								    	// root: Node::new(usvg::NodeKind::Group(Group {
								    	// 	id: "red_white".to_string(),
								    	// 	..Default::default()
								    	// })),

										// ..Default::default()
								    // });
							}
				        }
				    }
				}
			}
		}
	}
}

impl Polygon {
	fn new(size: Size) -> Self {
        let mut default = Polygon::default();

        // Overwrite values with struct parameters
        default.tree.0.size = usvg::Size::from_wh(size.width, size.height).unwrap();
        default.tree.0.view_box = usvg::ViewBox {
            rect: usvg::NonZeroRect::from_xywh(0., 0., size.width, size.height).unwrap(),

            aspect: usvg::AspectRatio::default(),
        };

        // TODO: create a reusable way to update filters

        // Update size of filter
		for node in default.tree.0.root.descendants().take(1) {
	        if let usvg::NodeKind::Group(ref mut g) = *node.borrow_mut() {
				for item in g.filters.iter_mut() {
            		*item = Rc::new(usvg::filter::Filter {
		                id: "".to_string(),
		                primitive_units: Units::UserSpaceOnUse,
		                primitives: item.primitives.clone(),
		                rect: usvg::NonZeroRect::from_xywh(0., 0., size.width, size.height).unwrap(),
		                units: Units::UserSpaceOnUse
            		});
				}
	        }
	    }

        default.size = size;
        default
	}

	fn update(&mut self, margin: &Margin, radius: Radius, stroke: Option<Stroke>, color: Option<Colors>) {

		// could do with replacing values, clear does not allocate new memory
		self.path_data.clear();

		self.margin = *margin;

		self.color.fill = color.as_ref().unwrap().fill;
		self.color.stroke = color.as_ref().unwrap().stroke;
		self.color.shadow = color.as_ref().unwrap().shadow;

		self.stroke.width = stroke.unwrap().width;

		self.select_curved_corners(radius.selection, radius.size);
	}
}

trait Calculate {
	fn shrink_layer_coord(&mut self, shrink: f32, smooth: Option<f32>) -> Points;
	fn select_curved_corners(&mut self, selection: Selection, radius: f32) {}
	fn construct_layer_paths(&mut self) {}
}

impl Calculate for Polygon {
	fn shrink_layer_coord(&mut self, shrink: f32, smooth: Option<f32>) -> Points {
		let mut points = match &self.margin {
			Margin::Four (l, t, r, d) => {
				Points {
					lb: { Point { x: *l + shrink, y: self.size.height - d - shrink, radius: self.points.lb.radius} },
					lt: { Point { x: *l + shrink, y: *t + shrink, radius: self.points.lt.radius} },
					rt: { Point { x: self.size.width - r - shrink, y: *t + shrink, radius: self.points.rt.radius} },
					rb: { Point { x: self.size.width - r - shrink, y: self.size.height - d - shrink, radius: self.points.rb.radius} },
				}
			}
			Margin::Three(l, t, r) => {
				Points {
					lb: { Point { x: *l + shrink, y: self.size.height - t - shrink, radius: self.points.lb.radius} },
					lt: { Point { x: *l + shrink, y: *t + shrink, radius: self.points.lt.radius} },
					rt: { Point { x: self.size.width - r - shrink, y: *t + shrink, radius: self.points.rt.radius} },
					rb: { Point { x: self.size.width - r - shrink, y: self.size.height - t - shrink, radius: self.points.rb.radius} },
				}
			}
			Margin::Two(x, y) => {
				Points {
					lb: { Point { x: *x + shrink, y: self.size.height - y - shrink, radius: self.points.lb.radius} },
					lt: { Point { x: *x + shrink, y: *y + shrink, radius: self.points.lt.radius} },
					rt: { Point { x: self.size.width - x - shrink, y: *y + shrink, radius: self.points.rt.radius} },
					rb: { Point { x: self.size.width - x - shrink, y: self.size.height - y - shrink, radius: self.points.rb.radius} },
				}
			}
			Margin::One(s) => {
				Points {
					lb: { Point { x: *s + shrink, y: self.size.height - s - shrink, radius: self.points.lb.radius} },
					lt: { Point { x: *s + shrink, y: *s + shrink, radius: self.points.lt.radius} },
					rt: { Point { x: self.size.width - s - shrink, y: *s + shrink, radius: self.points.rt.radius} },
					rb: { Point { x: self.size.width - s - shrink, y: self.size.height - s - shrink, radius: self.points.rb.radius} },
				}
			}
		};

		// Deducts from radius to fit snug inside a stroke layer
		if smooth.is_some() {

			// Conjugate corners when radius is smaller then stroke width
			if self.color.stroke.is_some() {
				// fills can bleed into stroke when values are between 0.0 - 1.0
				// add tests to prove that this is not the case

				if self.stroke.width / self.points.lb.radius < 1. {
					points.lb.radius -= smooth.unwrap();
				} else {
					points.lb.radius = 0.
				}
				if self.stroke.width / self.points.lt.radius < 1. {
					points.lt.radius -= smooth.unwrap();
				} else {
					points.lt.radius = 0.
				}
				if self.stroke.width / self.points.rt.radius < 1. {
					points.rt.radius -= smooth.unwrap();
				} else {
					points.rt.radius = 0.
				}
				if self.stroke.width / self.points.rb.radius < 1. {
					points.rb.radius -= smooth.unwrap();
				} else {
					points.rb.radius = 0.
				}
			}
		} else {
			points.lb.radius -= self.stroke.width/2.;
			points.lt.radius -= self.stroke.width/2.;
			points.rt.radius -= self.stroke.width/2.;
			points.rb.radius -= self.stroke.width/2.;
		}
		points
	}

	// Fill and Stroke path positions are calculated here with respect to each
	// other and the margins. We amend layer positions since the stroke is drawn
	// from the middle of a path.
	// See: https://developer.mozilla.org/en-US/docs/Web/SVG/Tutorial/Fills_and_Strokes#stroke

	// Example

	// Texture : (32, 32)
	// Margin  : (4, 4, 4, 4)
	// Stroke  : 1

	// Path Coordinates

	// Point starts from the (left, bottom)
	// Base    : (4, 28) (4, 4) (28, 4) (28, 28)
	// Fill    : (5, 27) (5, 5) (27, 5) (27, 27)
	// Stroke  : (4.5, 27.5) (4.5, 4.5) (27.5, 4.5) (27.5, 27.5)

	// Resizing path shapes are handled by margins and the intial dimension the
	// struct is set too. All path nodes should be placed within this region.
	// Strokes are inner aligned so everything apart from filters should be
	// within this region.
	// Margins are used to make space for effects that break past this region.
	// See: https://www.w3.org/TR/SVG/coords.html#BoundingBoxes

	fn construct_layer_paths(&mut self) {
		let mut layer = LayerProps { path: PathBuilder::default(), shrink: 0. };

		// stroke must be checked first so that fill can adjust if necessary
		if self.color.stroke.is_some() && self.stroke.width != 0. {
			// Position stroke from the inside of the path

			layer.shrink = self.stroke.width;

			// calculate coordinates at half the size of the stroke width to
			// align stroke from the inside
			let points =  self.shrink_layer_coord(layer.shrink/2., None);
			layer.path = self.draw(&points);

			self.path_data.push(Layer::Stroke(layer.clone()));
			layer.clear()
		}

		if self.color.fill.is_some() {

			if self.path_data.layers.borrow().len() != 0 {
				layer.shrink = self.stroke.width;
			}

			let points =  self.shrink_layer_coord(layer.shrink, Some(self.stroke.width));
			layer.path = self.draw(&points);

			self.path_data.push(Layer::Fill(layer.clone()));
			layer.clear()
		}

		if self.color.shadow.is_some() {

			let points =  self.shrink_layer_coord(layer.shrink+10., None);
			layer.path = self.draw(&points);

			self.path_data.push(Layer::Shadow(layer.clone()));
			layer.clear()
		}
	}

    fn select_curved_corners(&mut self, selection: Selection, radius: f32) {
        self.points.lb.radius = 0.;
        self.points.lt.radius = 0.;
        self.points.rt.radius = 0.;
        self.points.rb.radius = 0.;

        match selection {
            Selection::Left => {
                self.points.lb.radius = radius;
                self.points.lt.radius = radius
            }
            Selection::Top => {
                self.points.lt.radius = radius;
                self.points.rt.radius = radius
            }
            Selection::Right => {
                self.points.rt.radius = radius;
                self.points.rb.radius = radius
            }
            Selection::Bottom => {
                self.points.rb.radius = radius;
                self.points.lb.radius = radius
            }
            Selection::All => {
                self.points.lb.radius = radius;
                self.points.lt.radius = radius;
                self.points.rt.radius = radius;
                self.points.rb.radius = radius
            }
            Selection::Custom(lb, lt, rt, rb) => {
                self.points.lb.radius = lb;
                self.points.lt.radius = lt;
                self.points.rt.radius = rt;
                self.points.rb.radius = rb
            }
            Selection::None => {}
        }
    }
}

// contains methods that can either update the struct state or return structures
fn compute<T>(shape: &mut T)
where
	T: Calculate + Drawer
{
	// create layers that have a color assigned to them
	shape.construct_layer_paths();
	// append tree struct
	shape.construct_layer_nodes();
}

struct Radius {
	selection: Selection,
	size: f32
}

enum Selection {
	Left,
	Top,
	Right,
	Bottom,
	All,
	Custom(f32, f32, f32, f32),
	None
}

#[derive(Default, Debug)]
struct Colors {
	fill: Option<usvg::Color>,
	stroke: Option<usvg::Color>,
	shadow: Option<usvg::Color>,
}

#[derive(Default, Debug)]
pub struct Stroke {
	width: f32
}

pub enum Texture {
	Border,
	Background,
	Shadow,
	Indicator,

	Svg,
}

pub fn render_image() {
	// let mut shaper = Polygon::new( Size { width: 48., height: 48. });
	let mut shaper = Polygon::new( Size { width: 48., height: 16. });
	shaper.margin = Margin::Four(8., 3., 2., 3.);

    shaper.update(
        &Margin::Four(6., 1., 0., 1.),
        // &Margin::Four(8., 3., 2., 3.),
        // &Margin::Two(6., 1.),
        // &Margin::One(4.),
        Radius {
            // selection: Selection::Custom(0., 8., 20.0, 20.),
            // selection: Selection::None,
            selection: Selection::All,
            // selection: Selection::Left,
            size: 6.,
        },
        Some(Stroke {
        	width: 2.5
        }),
        Some(Colors {
        	fill: Some(Color { blue: 255,red: 255, green: 0 }),
	        stroke: Some(Color { blue: 0,red: 0, green: 155 }),
	        // shadow: Some(Color { blue: 0,red: 0, green: 0 }),

	        ..Default::default()
	    })
    );

    // shaper.update(
    //     // &Margin::Four(10., 10., 10., 10.),
    //     // &Margin::Two(32., 16.),
    //     &Margin::Two(3., 1.),
    //     Radius {
    //         // selection: Selection::Custom(250., 100., 350., 40.),
    //         selection: Selection::All,
    //         size: 3.,
    //     },
    //     Some(Stroke {
    //     	width: 4.
    //     }),
    //     Some(Colors {
    //     	fill: Some(Color { blue: 255, red: 255, green: 25 }),
	//         stroke: Some(Color { blue: 25, red: 255, green: 255 }),
	//         // shadow: Some(Color { blue:  255,red:  255, green: 255 }),

	//         ..Default::default()
	//     })
    // );

    compute(&mut shaper);

    // writer requires `svgtypes` crate, else it panics
    let s = shaper.tree.0.to_string(&XmlOptions::default());

	  let mut path = PathBuf::new();
	  path.push(env::current_dir().unwrap().parent().unwrap());
	  path.push("assets");
	  path.push("out.svg");

	// write svg
    let _ = crate::write_file(path.to_str().unwrap(), s.as_bytes());

	path.pop();

	render_png_scale(&shaper.tree.0, &[1, 2, 3, 20], &mut path);
}

fn render_png_scale(tree: &usvg::Tree, scales: &[u32], path: &mut PathBuf) {
	let size = tree.size.to_int_size();
	path.push("out");
	for scale in scales {
		let mut pixmap = tiny_skia::Pixmap::new(size.width() * scale, size.height() * scale).unwrap();
		// post_scale will return if params equal to default (1)
		resvg::Tree::render(
			&resvg::Tree::from_usvg(&tree),
			tiny_skia::Transform::default().pre_scale(*scale as f32, *scale as f32),
			&mut pixmap.as_mut(),
		);
		let prefix: String;
		if *scale == 1 {
			prefix = format!("{}.png",path.to_str().unwrap());
		} else {
			prefix = format!("{}@{}x.png",path.to_str().unwrap(),scale.to_string());
		}
		pixmap.as_ref().save_png(prefix).unwrap();
	};
}
