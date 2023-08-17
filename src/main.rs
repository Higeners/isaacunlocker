#![windows_subsystem = "windows"]
slint::slint! {
	import { GridBox , ScrollView, GroupBox, ListView, HorizontalBox, CheckBox} from "std-widgets.slint";
	import "./src/upheavtt.ttf";
	struct AchievementIcon {
		image: image,
		name: string,
	}

	export component Icon inherits Image {
		width: 60px;
		height: 60px;
		property <bool> has-unlocked;
		rect := Rectangle {
			callback pressed;
			ta:= TouchArea { 
				clicked() => {
					parent.pressed();
				}
			}
			pressed => {
				root.has-unlocked = !root.has-unlocked;
			}
			border-width: 0px;
			border-color: #b44160;

			animate border-width {duration: 100ms ; easing: ease-in;}
			states [ 
				unlocked when root.has-unlocked : {
					border-width: 3px;
					border-color: #4188b4;
				}
				inactive when !ta.has-hover && !root.has-unlocked : {
					border-width: 0px;
				
				}
				active when !root.has-unlocked && ta.has-hover: {
					border-width: 3px;
					border-color: #b44160;
				
				}
			]
		}
	} 
	component InputField {
		in property <length> font-size;
		in property <length> border-width;
		in property <string> input-title;
		in property <color> background-color;
		in property <color> font-color;
		out property <string> text;
		HorizontalBox {
			alignment: start;
			spacing: 10px;
			Text{
				font-size: root.font-size;
				text: input-title;
			}
			textin:= TextInput {
				wrap: word-wrap;
				single-line: false;
				font-size: root.font-size;
				color: font-color;
				edited => {text = self.text}
			}
			
		}
		Rectangle {
			z: -10;
			x: textin.x;
			background: background-color;
			clip : true;
			width: textin.width + 10px;
			height: textin.height + 12px;
			border-radius: 4px;
			border-width: root.border-width;
			border-color: gray.darker(10%);
		}
		
	}
	export component App inherits Window {
		title: "Isaac Achievement Unlocker";
		min-width: 200px;
		preferred-width: 400px;
		preferred-height: 200px;
		background: #202325;
		in property <[AchievementIcon]> icons: [
			{image: @image-url("images/Achievement_-0-_Baby_icon.png")},
			{image: @image-url("images/Achievement_-0-_Baby_icon.png")},
			{image: @image-url("images/Achievement_-0-_Baby_icon.png")},
			{image: @image-url("images/Achievement_-0-_Baby_icon.png")},
			{image: @image-url("images/Achievement_-0-_Baby_icon.png")},
			{image: @image-url("images/Achievement_-0-_Baby_icon.png")},
			{image: @image-url("images/Achievement_-0-_Baby_icon.png")},
			{image: @image-url("images/Achievement_-0-_Baby_icon.png")},
			{image: @image-url("images/Achievement_-0-_Baby_icon.png")},
			{image: @image-url("images/Achievement_-0-_Baby_icon.png")},
			{image: @image-url("images/Achievement_-0-_Baby_icon.png")},
			{image: @image-url("images/Achievement_-0-_Baby_icon.png")},
		];
		property <int> list-width: 16;//Math.max(Math.ceil(self.width / 100px), 2);
		property <int> list-height: Math.ceil(icons.length / list-width);
		Rectangle {
			background: transparent;
			width: input-tab.width;
			height: input-tab.height + self.border-width;
			x: input-tab.x;
			y: input-tab.y;
			border-width: 4px;
			border-radius: 16px;
			border-color: #454242;
		}
		VerticalLayout {
			input-tab:= VerticalLayout {
				InputField {
					input-title: "Search:";
					font-size: 40px;
					border-width: 2px;
					background-color: gray.darker(40%);
					font-color: white;
				}
				HorizontalBox {
					padding: 0px;
					spacing: 0px;
					alignment: start;
					InputField {
						input-title: "Range of achievements:";
						font-size: 40px;
						border-width: 2px;
						background-color: gray.darker(40%);
						font-color: white;
					}
					InputField {
						input-title: "-";
						font-size: 40px;
						border-width: 2px;
						background-color: gray.darker(40%);
						font-color: white;
					}
				}
				CheckBox { 
					text: "Unlocked";
				}
		}
			ListView  {
				for i in list-height : HorizontalLayout{
					padding: 4px;
					spacing: 4px;
					property <int> list_actual_width: Math.min(icons.length - i * list-width, list-width);
					for t in list_actual_width : VerticalLayout {
						property <int> index: t + i * list-width;
						width: 100px;
						Icon {
							
							source: icons[index].image;
						}
						Text {
							text: index + 1;
							font-weight: 500;
							font-size: 16px;
							font-family: "Upheaval TT (BRK)";
						}Text {
							text: icons[index].name;
							font-weight: 500;
							font-size: 16px;
							font-family: "Upheaval TT (BRK)";
							wrap: word-wrap;
						}
					}
				}
			}
	}
	}
}

fn imbed_images() -> Vec<(slint::Image, String)>{
	use include_dir::*;
	use slint::*;
	const IMAGES: Dir = include_dir!("./images");
	let mut images: Vec<(slint::Image, String)> = vec![];
	for file in IMAGES.files() {
		let image = image::load_from_memory(file.contents()).unwrap().into_rgb8();
		let buffer = SharedPixelBuffer::<Rgb8Pixel>::clone_from_slice(image.as_raw(), image.width(), image.height());
		let i = Image::from_rgb8(buffer);
		let s = file.path().to_str().unwrap();
		images.push((i, s.strip_prefix("Achievement_").unwrap().strip_suffix("_icon.png").unwrap().to_string()));
	}
	images
}

fn main() {
	use slint::Model;
	let images = imbed_images();
	let app = App::new().unwrap();
	let icons = std::rc::Rc::new(slint::VecModel::from(images.iter().fold(Vec::<AchievementIcon>::new(), | mut acc, (x,s)| {
		acc.push(AchievementIcon {image: x.clone(), name: s.into()});
		acc
	})));
	app.set_icons(icons.into());

	app.run().unwrap();
}
