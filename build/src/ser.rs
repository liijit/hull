use crate::external::{self, *};

use std::env;
use std::convert::AsRef;
use std::fmt::Debug;
use std::{collections::HashMap, f32};

use serde::Serialize;

use strum_macros::AsRefStr;

#[derive(Serialize, Debug, Clone)]
#[serde(transparent)]
pub struct Stack<T> {
	stack: Vec<T>,
}

impl<T> Stack<T>
where
	T: Debug + Clone,
{
	pub fn new() -> Self {
		Stack { stack: Vec::new() }
	}

	pub fn push(&mut self, item: T) {
		self.stack.push(item)
	}

	pub fn len(&self) -> usize {
		self.stack.len().clone()
	}

	pub fn pop(&mut self) -> Option<T> {
		self.stack.pop()
	}

	pub fn push_vec_of_items(&mut self, items: Vec<T>) {
		// consumes each item
		items.into_iter().for_each(|val| -> _ {
			self.push(val);
		});
	}
}

#[derive(Debug, Serialize, Clone)]
pub struct SublimeTheme {
	pub variables: Variables<'static>,
	pub rules: Rules<'static>,
}

#[derive(Debug, Serialize, Clone)]
#[serde(transparent)]
pub struct Rules<'a> {
	elements: Stack<Elements<'a>>,
}

#[derive(Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct GroupGeneric<T: Debug> {
	elements: Stack<T>,
}

#[derive(Debug, Serialize, Clone)]
#[serde(untagged)]
#[serde(rename_all = "snake_case")]
pub enum Elements<'a> {
	TitleBar(TitleBar<'a>),
	ContainerOpacity(ContainerOpacity),
	SidebarContainer(SidebarContainer<'a>),
	SidebarTree(SidebarTree<'a>),

	TitleBarGroup(GroupGeneric<TitleBar<'a>>),
	ContainerOpacityGroup(GroupGeneric<ContainerOpacity>),
	SidebarContainerGroup(GroupGeneric<SidebarContainer<'a>>),
	SidebarTreeGroup(GroupGeneric<SidebarTree<'a>>),
}

#[derive(Debug, Serialize, AsRefStr)]
pub enum ElementList {
	#[strum(serialize = "title_bar")]
	TitleBar,
	#[strum(serialize = "sidebar_container")]
	SidebarContainer,
	#[strum(serialize = "sidebar_tree")]
	SidebarTree,
}


#[derive(Debug, Serialize, Clone)]
pub struct GroupGenericIntoIterator<T: Debug> {
	elements: GroupGeneric<T>,
}

impl<T> Iterator for GroupGenericIntoIterator<T>
where
	T: Debug + Clone,
{
	type Item = T;

	fn next(&mut self) -> Option<Self::Item> {
		if self.elements.elements.len() == 0 {
			return None;
		}
		self.elements.elements.pop()
	}
}

impl<T> IntoIterator for GroupGeneric<T>
where
	T: Debug + Clone,
{
	type Item = T;
	type IntoIter = GroupGenericIntoIterator<T>;

	fn into_iter(self) -> GroupGenericIntoIterator<T> {
		GroupGenericIntoIterator { elements: self }
	}
}

impl From<std::fmt::Error> for SemanticError {
	fn from(error: std::fmt::Error) -> Self {
		SemanticError::Std(error)
	}
}

trait Morph<T> {
	fn generate(a: &T) -> Self
	where
		Self: Sized;
}

impl<'a> Morph<external::ElementSidebarTree> for GroupGeneric<SidebarTree<'a>> {
	fn generate(a: &ElementSidebarTree) -> Self
	where
		Self: Sized,
	{
		let mut stack: Vec<SidebarTree> = vec![];
		let _ = a.theme.iter().for_each(|f| match f {
			ElementSidebarTreeTheme::Origin(a) => a.variant.iter().for_each(|f| match f {
				ElementVariant::Origin(item) => {
					stack.push(SidebarTree {
						indent: Some(item.indent),
						indent_offset: Some(item.indent_offset),
						row_padding: Some(item.row_padding),
						indent_top_level: Some(item.indent_top_level),
						dark_content: Some(item.dark_content),
						spacer_rows: Some(item.spacer_rows),
						..Default::default()
					});
				}
				ElementVariant::Misc(item) => {
					stack.push(SidebarTree {
						indent_offset: item.indent_offset,
						settings: item.settings.clone(),
						row_padding: item.row_padding,
						dark_content: item.dark_content,
						spacer_rows: item.spacer_rows,
						..Default::default()
					});
				}
				ElementVariant::Unknown(e) => {
					panic!("Panic occured due to unexpected field: \n {:#?}", e)
				}
			}),
			ElementSidebarTreeTheme::Sublime(_) => {}
			ElementSidebarTreeTheme::Unknown(e) => {
				panic!("Panic occured due to unexpected field: \n {:#?}", e)
			}
		});

		stack.reverse();

		GroupGeneric {
			elements: Stack { stack },
		}
	}
}

impl<'a> Default for GroupGeneric<SidebarTree<'a>> {
	fn default() -> Self {
		GroupGeneric {
			elements: Stack {
				stack: vec![SidebarTree {
					..Default::default()
				}],
			},
		}
	}
}

impl<'a> Morph<external::ElementSidebarContainer> for GroupGeneric<SidebarContainer<'a>> {
	fn generate(item: &ElementSidebarContainer) -> Self {
		GroupGeneric {
			elements: Stack {
				stack: vec![SidebarContainer {
					settings: Some(vec![TitleBarSettings::ThemedTitleBar]),
					..Default::default()
				}],
			},
		}
	}
}

impl<'a> Default for GroupGeneric<TitleBar<'a>> {
	fn default() -> Self {
		GroupGeneric {
			elements: Stack {
				stack: vec![
					TitleBar {
						settings: Some(vec![TitleBarSettings::ThemedTitleBar]),
						attributes: None,
						fg: Some("rgba(255, 255, 255, 0.7)"),
						bg: Some("var(--background)"),
						style: Some(Style::Dark),
						..Default::default()
					},
					TitleBar {
						settings: Some(vec![TitleBarSettings::ThemedTitleBar]),
						attributes: Some(vec![Luminosity::FileLight]),
						fg: Some("rgba(0, 0, 0, 0.7)"),
						bg: Some("var(--background)"),
						style: Some(Style::Light),
						..Default::default()
					},
				],
			},
		}
	}
}

impl<'a> Default for GroupGeneric<ContainerOpacity> {
	fn default() -> Self {
		GroupGeneric {
			elements: Stack {
				stack: vec![
					ContainerOpacity {
						layer0_opacity: 0.,
						layer1_opacity: 0.,
						layer2_opacity: 0.,
						layer3_opacity: 0.,
					},
					ContainerOpacity {
						layer0_opacity: 0.,
						layer1_opacity: 0.,
						layer2_opacity: 0.,
						layer3_opacity: 0.,
					},
					ContainerOpacity {
						layer0_opacity: 0.,
						layer1_opacity: 0.,
						layer2_opacity: 0.,
						layer3_opacity: 0.,
					},
				],
			},
		}
	}
}

#[derive(Serialize, Debug, Clone)]
pub struct Variables<'a>(HashMap<&'a str, &'a str>);

impl<'a> Variables<'a> {
	pub fn new() -> Self {
		let mut t = HashMap::new();

		// example
		t.insert(
			"tree_row_margin_y_1_shadow",
			"Hull Theme/textures/sidebar/tree_row/shadow_r_1_m_6_1.png",
		);

		Variables(t)
	}
}

impl<'a> Rules<'a> {
	pub fn new() -> Self {
		let mut stack = Stack::<Elements>::new();

		let mut path = crate::dir::PathConstructor::new();
		path.set_parent_directory(crate::dir::PathLocations::Build, "");
		path.set_filename("options");
		path.set_extension("jsonc");
		crate::dir::PathBuilder::create_dir(&path);

		let a = load_model(path.get_full_path()).unwrap();

		// Structs of elements are returned from default and impl functions.

		// Defaults can include an array of elements defined in a generic
		// function that are consumed and pushed onto the stack. Or you can use
		// `generate` modify structs based on the `options.jsonc` given.

		GroupGeneric::<TitleBar>::default()
			.into_iter()
			.for_each(|x| stack.push(Elements::TitleBar(x)));

		GroupGeneric::<SidebarContainer>::generate(&a.elements.sidebar_container)
			.into_iter()
			.for_each(|x| stack.push(Elements::SidebarContainer(x)));

		GroupGeneric::<SidebarTree>::generate(&a.elements.sidebar_tree)
			.into_iter()
			.for_each(|x| stack.push(Elements::SidebarTree(x)));

		Rules { elements: stack }
	}
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum TitleBarSettings {
	ThemedTitleBar,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Luminosity {
	FileLight,
	FileMedium,
	FileMediumDark,
	FileDark,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Style {
	Light,
	Dark,
}

#[derive(Debug, Serialize, Clone)]
pub struct TitleBar<'a> {
	pub class: &'a str,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub settings: Option<Vec<TitleBarSettings>>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub attributes: Option<Vec<Luminosity>>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub bg: Option<&'a str>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub fg: Option<&'a str>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub style: Option<Style>,
}

impl Default for TitleBar<'_> {
	fn default() -> Self {
		Self {
			// maybe we can refernce the elements name instead of referring
			// to newely created enum
			class: ElementList::TitleBar.as_ref(),
			settings: Default::default(),
			attributes: Default::default(),
			bg: Default::default(),
			fg: Default::default(),
			style: Default::default(),
		}
	}
}

#[derive(Debug, Serialize, Clone)]
pub struct SidebarContainer<'a> {
	pub class: &'a str,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub settings: Option<Vec<TitleBarSettings>>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub layers: Option<Layers<'a>>,
}

impl Default for SidebarContainer<'_> {
	fn default() -> Self {
		Self {
			class: ElementList::SidebarContainer.as_ref(),
			settings: Default::default(),
			layers: Default::default(),
		}
	}
}

#[derive(Debug, Serialize, Clone)]
enum ParentAttributes {
	FileDark,
	FileMediumDark,
	FileMedium,
	FileLight,

	Hover,
	Selected,
	Expanded,
	Expandable,
	Scrollable,

	Disabled,

	Dirty,

	Untracked,
	Modified,
	Missing,
	Staged,
	Added,
	Deleted,
	Unmerged,
	Ignored,
}

#[derive(Debug, Serialize, Clone)]
enum ParentClass {
	Window,
	SidebarContainer,
	StatusBar,
	SwitchProjectWindow,

	EditWindow,

	QuickPanel,

	TreeRow,
	CheckboxControl,

	FileSystemEntry,

	TabControl,
	ButtonControl,
	IconButtonControl,
	OverlayControl,
	TextLineControl,
	TextAreaControl,
	ScrollAreaControl,
	OverlayControlKindInfo,
	RadioButtonListControl,
	PopupControl,

	AutoCompletePopup,
	AutoComplete,

	KindContainer,

	KindFunction,
	KindKeyword,
	KindMarkup,
	KindNamespace,
	KindNavigation,
	KindSnippet,
	KindType,
	KindVariable,
	KindColorRedish,
	KindColorOrangish,
	KindColorYellowish,
	KindColorGreenish,
	KindColorCyanish,
	KindColorBluish,
	KindColorPurplish,
	KindColorPinkish,
	KindColorDark,
	KindColorLight,
}

#[derive(Debug, Serialize, Clone)]
struct Parents<'a> {
	class: &'a str,
	attributes: Option<Vec<ParentAttributes>>,
}

impl<'a> Default for Parents<'a> {
	fn default() -> Self {
		Self {
			class: "",
			attributes: Some(vec![]),
		}
	}
}

#[derive(Debug, Serialize, Clone)]
pub struct SidebarTree<'a> {
	class: &'a str,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	row_padding: Option<ArrayU8Size>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	parents: Option<Vec<Parents<'a>>>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	settings: Option<ElementCustomPreferences>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	indent: Option<u8>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	indent_offset: Option<u8>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	indent_top_level: Option<bool>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	dark_content: Option<bool>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	spacer_rows: Option<bool>,
}

impl Default for SidebarTree<'_> {
	fn default() -> Self {
		Self {
			class: ElementList::SidebarTree.as_ref(),
			settings: Default::default(),
			row_padding: Default::default(),
			parents: Default::default(),
			indent: Default::default(),
			indent_offset: Default::default(),
			indent_top_level: Default::default(),
			dark_content: Default::default(),
			spacer_rows: Default::default(),
		}
	}
}

#[derive(Debug, Serialize, Clone)]
pub struct Layers<'a> {
	#[serde(default, skip_serializing_if = "Option::is_none")]
	opacity: Option<Opacity>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	texture: Option<Texture<'a>>,
}

impl Default for Layers<'_> {
	fn default() -> Self {
		Self {
			texture: Default::default(),
			opacity: Default::default(),
		}
	}
}

#[derive(Debug, Serialize, Clone, Default)]
struct Opacity {
	opacity0: f32,
	opacity1: f32,
	opacity2: f32,
	opacity3: f32,
}

#[derive(Debug, Serialize, Clone, Default)]
struct Texture<'a> {
	texture0: &'a str,
	texture1: &'a str,
	texture2: &'a str,
	texture3: &'a str,
}

// Applies to all UI elements
pub enum UIProperties {
	Opacity(ContainerOpacity),
}

#[derive(Serialize, Clone, Debug)]
pub struct ContainerOpacity {
	pub layer0_opacity: f32,
	pub layer1_opacity: f32,
	pub layer2_opacity: f32,
	pub layer3_opacity: f32,
}
