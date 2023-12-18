use json_comments::StripComments;
use monostate::MustBe;
use serde::{Deserialize, Serialize};
use std::{io, path::Path};

#[derive(Debug)]
pub enum SemanticError {
	Std(std::fmt::Error),
	IO(io::Error),
	Parse(serde_json::Error),
}

impl From<io::Error> for SemanticError {
	fn from(error: io::Error) -> Self {
		SemanticError::IO(error)
	}
}

impl From<serde_json::Error> for SemanticError {
	fn from(error: serde_json::Error) -> Self {
		SemanticError::Parse(error)
	}
}

pub fn load_model<P: AsRef<Path> + std::fmt::Debug>(path: P) -> Result<Model, SemanticError> {

	let data = std::fs::read_to_string(path)?;
	// strip out any /* */ and // comments so that serde doesn't error out
	let data = StripComments::new(data.as_bytes());

	// reads into data structure
	let model: Model = serde_json::from_reader(data)?;

	Ok(model)
}

#[derive(Debug, PartialEq, Deserialize, Clone)]
#[serde(untagged)]
pub enum Unknown {}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(untagged)]
pub enum ArrayU8Size {
	ArraySize4([i8; 4]),
	ArraySize2([i8; 2]),
	ArraySize1([i8; 1]),
}

impl Default for ArrayU8Size {
	fn default() -> Self {
	    Self::ArraySize1([0])
	}
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ArrayStringSize {
	ArraySize4([String; 4]),
	ArraySize2([String; 2]),
	ArraySize1([String; 1]),
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum BoolOrArray {
	Array(Vec<f32>),
	Boolean(bool),
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct LuminosityVariantGeneric<T> {
	pub light: T,
	pub medium: T,
	pub medium_dark: T,
	pub dark: T,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ElementVariant<T, U> {
	Origin(T),
	Misc(U),
	Unknown(serde_json::Value),
}

#[derive(Deserialize, Debug, Clone)]
pub struct Model {
	pub variables: Variables,
	pub elements: Elements,
	pub theme: Theme,
}

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct Theme {
	pub name: String,
}

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)]
pub struct Variables {
	pub fonts: Fonts,
	pub colors: Colors,
	pub base_elements: BaseElements,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct Fonts {
	pub size: FontSize,
	pub elements: FontElements,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct FontElements {
	pub sidebar_heading: String,
	pub sidebar_label: String,
	pub vcs_changes_annotation: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct FontSize {
	pub xs: i8,
	pub sm: i8,
	pub md: i8,
	pub lg: i8,
	pub xl: i8,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct Colors {
	red: String,
	green: String,
	blue: String,
	blue2: String,
	blue3: String,
	blue4: String,
	blue5: String,
	blue6: String,
	yellow: String,
	yellow2: String,
	black: String,
	grey: String,
	find: String,
	find2: String,
	orange: String,
	pink: String,
	white: String,
	white2: String,
	white3: String,
	white4: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct BaseElements {
	pub container: Opacity,
	pub row: Opacity,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct Elements {
	pub sidebar_container: ElementSidebarContainer,
	pub sidebar_tree: ElementSidebarTree,
	pub tree_row: ElementTreeRow,
}

// Sidebar Container

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct ElementSidebarContainer {
	pub theme: Vec<ElementSidebarContainerTheme>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ElementSidebarContainerTheme {
	Default(ElementSidebarContainerThemeDefault),
	Sublime(ElementSidebarContainerThemeSublime),
	Unknown {},
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct ElementSidebarContainerThemeDefault {
	pub name: MustBe!("default"),
	pub color: LuminosityVariantGeneric<ElementSidebarContainerColor>,
	pub opacity: LuminosityVariantGeneric<ElementSidebarContainerOpacity>,
	pub variant: Vec<
		ElementVariant<
			ElementSidebarContainerVariantDefault,
			ElementSidebarContainerVariantMisc,
		>,
	>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct ElementSidebarContainerVariantDefault {
	pub origin: MustBe!(true),
	pub content_margin: ArrayU8Size,
	pub inner_margin: ArrayU8Size,
	pub texture_position: ArrayU8Size,
	pub radius: Vec<f32>,
	pub radius_corners: ArrayStringSize,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct ElementSidebarContainerVariantMisc {
	pub content_margin: ArrayU8Size,
	pub inner_margin: ArrayU8Size,
	pub texture_position: ArrayU8Size,
	pub radius: Vec<f32>,
	pub radius_corners: ArrayStringSize,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct ElementSidebarContainerThemeSublime {
	pub name: MustBe!("sublime"),
	pub color: LuminosityVariantGeneric<ElementSidebarContainerThemeSublimeColor>,
	pub opacity: LuminosityVariantGeneric<ElementSidebarContainerThemeSublimeOpacity>,
	pub variant: Vec<
		ElementVariant<
			ElementSidebarContainerVariantSublime,
			ElementSidebarContainerVariantSublimeMisc,
		>,
	>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct ElementSidebarContainerVariantSublime {
	pub origin: MustBe!(true),
	pub content_margin: ArrayU8Size,
	pub inner_margin: ArrayU8Size,
	pub texture_position: ArrayU8Size,
	pub radius: Vec<f32>,
	pub radius_corners: ArrayStringSize,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct ElementSidebarContainerVariantSublimeMisc {
	pub content_margin: ArrayU8Size,
	pub inner_margin: ArrayU8Size,
	pub texture_position: ArrayU8Size,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct ElementSidebarContainerThemeOpacity {
	pub element_bg: ElementSidebarContainerOpacityState,
	pub shadow: ElementSidebarContainerOpacityState,
	pub border: ElementSidebarContainerOpacityState,
	pub bg: ElementSidebarContainerOpacityState,
	pub animation: ElementAnimationState,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct ElementSidebarContainerThemeSublimeColor {
	pub element_bg: String,
	pub border: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct ElementSidebarContainerThemeSublimeOpacity {
	pub element_bg: ElementSidebarContainerOpacityState,
	pub border: ElementSidebarContainerOpacityState,
	pub animation: ElementAnimationState,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ElementSidebarContainerVariantMap {
	Default(ElementSidebarContainerVariantDefault),
	Sublime(ElementSidebarContainerVariantSublime),
	// Unknown{}
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct ElementSidebarContainerOpacity {
	pub element_bg: ElementSidebarContainerOpacityState,
	pub shadow: ElementSidebarContainerOpacityState,
	pub border: ElementSidebarContainerOpacityState,
	pub bg: ElementSidebarContainerOpacityState,
	pub animation: ElementAnimationState,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct ElementSidebarContainerOpacityState {
	pub idle: f32,
	pub hover: f32,
	pub selected: f32,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct ElementSidebarContainerColor {
	pub element_bg: String,
	pub shadow: String,
	pub border: String,
	pub bg: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub enum Margin {
	Vec(i8, i8, i8, i8),
}

// Sidebar Tree
#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct ElementSidebarTree {
	pub theme: Vec<ElementSidebarTreeTheme>,
}

// Sidebar Tree
#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ElementSidebarTreeTheme {
	Origin(ElementSidebarTreeOrigin),
	Sublime(ElementSidebarTreeSublime),
	Unknown(serde_json::Value),
}

// Sidebar Tree
#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct ElementSidebarTreeOrigin {
	pub variant: Vec<ElementVariant<ElementSidebarTreeVariantOrigin, ElementSidebarTreeVariantMisc>>
}

// Sidebar Tree
#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct ElementSidebarTreeSublime {
	pub variant: Vec<ElementSidebarTreeVariantMapSublime>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ElementSidebarTreeVariantMapOrigin {
	Default(ElementSidebarTreeVariantOrigin),
	Misc(ElementSidebarTreeVariantMisc),
	Unknown(serde_json::Value),
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ElementSidebarTreeVariantOrigin {
	pub origin: MustBe!(true),
	pub options: ElementCustomPreferences,
	pub indent: u8,
	pub indent_offset: u8,
	pub row_padding: ArrayU8Size,
	pub indent_top_level: bool,
	pub dark_content: bool,
	pub spacer_rows: bool,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ElementSidebarTreeVariantMisc {
	pub settings: Option<ElementCustomPreferences>,
	pub indent_offset: Option<u8>,
	pub row_padding: Option<ArrayU8Size>,
	pub platforms: Option<Vec<String>>,
	pub parents: Option<Vec<ElementParentObject>>,
	pub dark_content: Option<bool>,
	pub spacer_rows: Option<bool>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ElementParentObject {
	Default(ParentObjectDefault),
	Unknown {},
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct ParentObjectDefault {
	pub class: Option<String>,
	pub attributes: Option<Vec<String>>,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ElementCustomPreferences {
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub hull_disclosure_button_control: Option<bool>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub hull_sidebar_tree: Option<Vec<String>>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ElementSidebarTreeVariantMapSublime {
	Default(ElementSidebarTreeVariantOrigin),
	Misc(ElementSidebarTreeVariantMisc),
	Unknown {},
}

// Tree Row

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct ElementTreeRow {
	pub theme: Vec<ElementTreeRowTheme>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ElementTreeRowTheme {
	Default(ElementTreeRowThemeDefault),
	Sublime(ElementTreeRowThemeSublime),
	Unknown {},
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct ElementTreeRowThemeDefault {
	pub name: MustBe!("default"),
	pub color: ElementTreeRowVariantColor,
	pub opacity: ElementTreeRowLuminosityOpacity,
	pub variant: Vec<ElementTreeRowVariantMapDefault>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct ElementTreeRowThemeSublime {
	pub name: MustBe!("sublime"),
	pub color: ElementTreeRowVariantColor,
	pub opacity: ElementTreeRowLuminosityOpacity,
	pub variant: Vec<ElementTreeRowVariantMapSublime>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct ElementTreeRowVariantColor {
	pub light: ElementTreeRowColor,
	pub medium: ElementTreeRowColor,
	pub medium_dark: ElementTreeRowColor,
	pub dark: ElementTreeRowColor,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct ElementTreeRowColor {
	pub shadow: String,
	pub border: String,
	pub bg: String,
	pub indicator: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct ElementTreeRowLuminosityOpacity {
	pub light: ElementTreeRowOpacity,
	pub medium: ElementTreeRowOpacity,
	pub medium_dark: ElementTreeRowOpacity,
	pub dark: ElementTreeRowOpacity,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct ElementTreeRowOpacity {
	pub shadow: ElementTreeRowOpacityState,
	pub border: ElementTreeRowOpacityState,
	pub bg: ElementTreeRowOpacityState,
	pub indicator: ElementTreeRowOpacityState,
	pub animation: ElementAnimationState,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct ElementTreeRowOpacityState {
	pub idle: f32,
	pub hover: f32,
	pub selected: f32,
	pub multiple: f32,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ElementTreeRowVariantMapSublime {
	Default(ElementTreeRowVariantSublimeDefault),
	Misc(ElementTreeRowVariantSublimeMisc),
	Unknown {},
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ElementTreeRowVariantMapDefault {
	Default(ElementTreeRowVariantDefault),
	Misc(ElementTreeRowVariantDefaultMisc),
	Unknown {},
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct ElementTreeRowVariantDefault {
	pub origin: MustBe!(true),
	pub radius: Vec<f32>,
	pub indicator: Vec<f32>,
	pub radius_corners: ArrayStringSize,
	pub inner_margin: ArrayU8Size,
	pub texture_margin: ArrayU8Size,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ElementTreeRowVariantDefaultMisc {
	pub inner_margin: Option<BoolOrArray>,
	pub texture_margin: ArrayU8Size,
	pub radius: Option<BoolOrArray>,
	pub radius_corners: Option<ArrayStringSize>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ElementTreeRowVariantSublimeDefault {
	pub origin: MustBe!(true),
	pub inner_margin: BoolOrArray,
	pub texture_margin: ArrayU8Size,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct ElementTreeRowVariantSublimeMisc {
	pub inner_margin: BoolOrArray,
	pub texture_margin: ArrayU8Size,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct Opacity {
	pub opacity: LuminosityVariantGeneric<ElementOpacity>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct ElementOpacity {
	pub shadow: ElementOpacityState,
	pub bg: ElementOpacityState,
	pub border: ElementOpacityState,
	pub indicator: ElementOpacityState,
	pub animation: ElementAnimationState,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct ElementAnimationState {
	pub interpolation: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct ElementOpacityState {
	pub idle: f32,
	pub hover: f32,
	pub selected: f32,
	pub multiple: f32,
}
