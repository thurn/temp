#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlexColor {
    /// Red color component, specified in the range 0.0 to 1.0 inclusive.
    #[prost(float, tag = "1")]
    pub red: f32,
    /// Green color component, specified in the range 0.0 to 1.0 inclusive.
    #[prost(float, tag = "2")]
    pub green: f32,
    /// Blue color component, specified in the range 0.0 to 1.0 inclusive.
    #[prost(float, tag = "3")]
    pub blue: f32,
    /// Alpha color component, specified in the range 0.0 (transparent) to 1.0
    /// (opaque) inclusive.
    #[prost(float, tag = "4")]
    pub alpha: f32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpriteAddress {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenderTextureAddress {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeBackground {
    #[prost(oneof = "node_background::BackgroundAddress", tags = "1, 2")]
    pub background_address: ::core::option::Option<node_background::BackgroundAddress>,
}
/// Nested message and enum types in `NodeBackground`.
pub mod node_background {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum BackgroundAddress {
        #[prost(message, tag = "1")]
        Sprite(super::SpriteAddress),
        #[prost(message, tag = "2")]
        RenderTexture(super::RenderTextureAddress),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FontAddress {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProjectileAddress {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EffectAddress {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AudioClipAddress {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlexVector2 {
    #[prost(float, tag = "1")]
    pub x: f32,
    #[prost(float, tag = "2")]
    pub y: f32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlexVector3 {
    #[prost(float, tag = "1")]
    pub x: f32,
    #[prost(float, tag = "2")]
    pub y: f32,
    #[prost(float, tag = "3")]
    pub z: f32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Dimension {
    #[prost(enumeration = "DimensionUnit", tag = "1")]
    pub unit: i32,
    #[prost(float, tag = "2")]
    pub value: f32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DimensionGroup {
    #[prost(message, optional, tag = "1")]
    pub top: ::core::option::Option<Dimension>,
    #[prost(message, optional, tag = "2")]
    pub right: ::core::option::Option<Dimension>,
    #[prost(message, optional, tag = "3")]
    pub bottom: ::core::option::Option<Dimension>,
    #[prost(message, optional, tag = "4")]
    pub left: ::core::option::Option<Dimension>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BorderWidth {
    #[prost(float, tag = "1")]
    pub top: f32,
    #[prost(float, tag = "2")]
    pub right: f32,
    #[prost(float, tag = "3")]
    pub bottom: f32,
    #[prost(float, tag = "4")]
    pub left: f32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BorderColor {
    #[prost(message, optional, tag = "1")]
    pub top: ::core::option::Option<FlexColor>,
    #[prost(message, optional, tag = "2")]
    pub right: ::core::option::Option<FlexColor>,
    #[prost(message, optional, tag = "3")]
    pub bottom: ::core::option::Option<FlexColor>,
    #[prost(message, optional, tag = "4")]
    pub left: ::core::option::Option<FlexColor>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BorderRadius {
    #[prost(message, optional, tag = "1")]
    pub top_left: ::core::option::Option<Dimension>,
    #[prost(message, optional, tag = "2")]
    pub top_right: ::core::option::Option<Dimension>,
    #[prost(message, optional, tag = "3")]
    pub bottom_right: ::core::option::Option<Dimension>,
    #[prost(message, optional, tag = "4")]
    pub bottom_left: ::core::option::Option<Dimension>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlexRotate {
    #[prost(float, tag = "1")]
    pub degrees: f32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlexTranslate {
    #[prost(message, optional, tag = "1")]
    pub x: ::core::option::Option<Dimension>,
    #[prost(message, optional, tag = "2")]
    pub y: ::core::option::Option<Dimension>,
    #[prost(float, tag = "3")]
    pub z: f32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlexScale {
    #[prost(message, optional, tag = "1")]
    pub amount: ::core::option::Option<FlexVector3>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextShadow {
    #[prost(message, optional, tag = "1")]
    pub offset: ::core::option::Option<FlexVector2>,
    #[prost(float, tag = "2")]
    pub blur_radius: f32,
    #[prost(message, optional, tag = "3")]
    pub color: ::core::option::Option<FlexColor>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TimeValue {
    #[prost(uint32, tag = "1")]
    pub milliseconds: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageSlice {
    #[prost(uint32, tag = "1")]
    pub top: u32,
    #[prost(uint32, tag = "2")]
    pub right: u32,
    #[prost(uint32, tag = "3")]
    pub bottom: u32,
    #[prost(uint32, tag = "4")]
    pub left: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlexStyle {
    #[prost(enumeration = "FlexAlign", tag = "1")]
    pub align_content: i32,
    #[prost(enumeration = "FlexAlign", tag = "2")]
    pub align_items: i32,
    #[prost(enumeration = "FlexAlign", tag = "3")]
    pub align_self: i32,
    #[prost(message, optional, tag = "4")]
    pub background_color: ::core::option::Option<FlexColor>,
    #[prost(message, optional, tag = "5")]
    pub background_image: ::core::option::Option<NodeBackground>,
    #[prost(message, optional, tag = "6")]
    pub border_color: ::core::option::Option<BorderColor>,
    #[prost(message, optional, tag = "7")]
    pub border_radius: ::core::option::Option<BorderRadius>,
    #[prost(message, optional, tag = "8")]
    pub border_width: ::core::option::Option<BorderWidth>,
    #[prost(message, optional, tag = "9")]
    pub inset: ::core::option::Option<DimensionGroup>,
    #[prost(message, optional, tag = "10")]
    pub color: ::core::option::Option<FlexColor>,
    #[prost(enumeration = "FlexDisplayStyle", tag = "11")]
    pub display: i32,
    #[prost(message, optional, tag = "12")]
    pub flex_basis: ::core::option::Option<Dimension>,
    #[prost(enumeration = "FlexDirection", tag = "13")]
    pub flex_direction: i32,
    #[prost(message, optional, tag = "14")]
    pub flex_grow: ::core::option::Option<f32>,
    #[prost(message, optional, tag = "15")]
    pub flex_shrink: ::core::option::Option<f32>,
    #[prost(enumeration = "FlexWrap", tag = "16")]
    pub wrap: i32,
    #[prost(message, optional, tag = "17")]
    pub font_size: ::core::option::Option<Dimension>,
    #[prost(message, optional, tag = "18")]
    pub height: ::core::option::Option<Dimension>,
    #[prost(enumeration = "FlexJustify", tag = "19")]
    pub justify_content: i32,
    #[prost(message, optional, tag = "20")]
    pub letter_spacing: ::core::option::Option<Dimension>,
    #[prost(message, optional, tag = "21")]
    pub margin: ::core::option::Option<DimensionGroup>,
    #[prost(message, optional, tag = "22")]
    pub max_height: ::core::option::Option<Dimension>,
    #[prost(message, optional, tag = "23")]
    pub max_width: ::core::option::Option<Dimension>,
    #[prost(message, optional, tag = "24")]
    pub min_height: ::core::option::Option<Dimension>,
    #[prost(message, optional, tag = "25")]
    pub min_width: ::core::option::Option<Dimension>,
    #[prost(message, optional, tag = "26")]
    pub opacity: ::core::option::Option<f32>,
    #[prost(enumeration = "FlexOverflow", tag = "27")]
    pub overflow: i32,
    #[prost(message, optional, tag = "28")]
    pub padding: ::core::option::Option<DimensionGroup>,
    #[prost(enumeration = "FlexPosition", tag = "29")]
    pub position: i32,
    #[prost(message, optional, tag = "30")]
    pub rotate: ::core::option::Option<FlexRotate>,
    #[prost(message, optional, tag = "31")]
    pub scale: ::core::option::Option<FlexScale>,
    #[prost(enumeration = "TextOverflow", tag = "32")]
    pub text_overflow: i32,
    #[prost(message, optional, tag = "33")]
    pub text_shadow: ::core::option::Option<TextShadow>,
    #[prost(message, optional, tag = "34")]
    pub transform_origin: ::core::option::Option<FlexTranslate>,
    #[prost(message, repeated, tag = "35")]
    pub transition_delays: ::prost::alloc::vec::Vec<TimeValue>,
    #[prost(message, repeated, tag = "36")]
    pub transition_durations: ::prost::alloc::vec::Vec<TimeValue>,
    #[prost(string, repeated, tag = "37")]
    pub transition_properties: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(enumeration = "EasingMode", repeated, tag = "38")]
    pub transition_easing_modes: ::prost::alloc::vec::Vec<i32>,
    #[prost(message, optional, tag = "39")]
    pub translate: ::core::option::Option<FlexTranslate>,
    #[prost(message, optional, tag = "40")]
    pub background_image_tint_color: ::core::option::Option<FlexColor>,
    #[prost(enumeration = "ImageScaleMode", tag = "41")]
    pub background_image_scale_mode: i32,
    #[prost(message, optional, tag = "42")]
    pub font: ::core::option::Option<FontAddress>,
    #[prost(enumeration = "FontStyle", tag = "43")]
    pub font_style: i32,
    #[prost(enumeration = "OverflowClipBox", tag = "44")]
    pub overflow_clip_box: i32,
    #[prost(message, optional, tag = "45")]
    pub paragraph_spacing: ::core::option::Option<Dimension>,
    #[prost(message, optional, tag = "46")]
    pub image_slice: ::core::option::Option<ImageSlice>,
    #[prost(enumeration = "TextAlign", tag = "47")]
    pub text_align: i32,
    #[prost(message, optional, tag = "48")]
    pub text_outline_color: ::core::option::Option<FlexColor>,
    #[prost(message, optional, tag = "49")]
    pub text_outline_width: ::core::option::Option<f32>,
    #[prost(enumeration = "TextOverflowPosition", tag = "50")]
    pub text_overflow_position: i32,
    #[prost(enumeration = "FlexVisibility", tag = "51")]
    pub visibility: i32,
    #[prost(enumeration = "WhiteSpace", tag = "52")]
    pub white_space: i32,
    #[prost(message, optional, tag = "53")]
    pub width: ::core::option::Option<Dimension>,
    #[prost(message, optional, tag = "54")]
    pub word_spacing: ::core::option::Option<Dimension>,
    #[prost(enumeration = "FlexPickingMode", tag = "55")]
    pub picking_mode: i32,
    #[prost(enumeration = "BackgroundImageAutoSize", tag = "56")]
    pub background_image_auto_size: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Flexbox {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Text {
    #[prost(string, tag = "1")]
    pub label: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScrollBar {
    #[prost(message, optional, tag = "1")]
    pub style: ::core::option::Option<FlexStyle>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScrollViewNode {
    /// The amount of elasticity to use when a user tries to scroll past
    /// the boundaries of the scroll view.
    #[prost(message, optional, tag = "1")]
    pub elasticity: ::core::option::Option<f32>,
    /// Controls the scrolling speed of the horizontal scrollbar.
    #[prost(message, optional, tag = "2")]
    pub horizontal_page_size: ::core::option::Option<f32>,
    /// Horizontal scrollbar
    #[prost(message, optional, tag = "3")]
    pub horizontal_scroll_bar: ::core::option::Option<ScrollBar>,
    /// Specifies whether the horizontal scroll bar is visible.
    #[prost(enumeration = "ScrollBarVisibility", tag = "4")]
    pub horizontal_scroll_bar_visibility: i32,
    /// Controls the rate at which the scrolling movement slows after a user
    /// scrolls using a touch interaction.
    #[prost(message, optional, tag = "5")]
    pub scroll_deceleration_rate: ::core::option::Option<f32>,
    /// The behavior to use when a user tries to scroll past the boundaries of
    /// the ScrollView content using a touch interaction.
    #[prost(enumeration = "TouchScrollBehavior", tag = "6")]
    pub touch_scroll_behavior: i32,
    /// Controls the scrolling speed of the vertical scrollbar.
    #[prost(message, optional, tag = "7")]
    pub vertical_page_size: ::core::option::Option<f32>,
    /// Vertical scrollbar
    #[prost(message, optional, tag = "8")]
    pub vertical_scroll_bar: ::core::option::Option<ScrollBar>,
    /// Specifies whether the vertical scroll bar is visible.
    #[prost(enumeration = "ScrollBarVisibility", tag = "9")]
    pub vertical_scroll_bar_visibility: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DraggableNode {
    /// Identifiers of DropTargetNodes that are valid drop targets for this
    /// draggable.
    #[prost(string, repeated, tag = "1")]
    pub drop_target_identifiers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Node to change the drag indicator to when this draggable is over a
    /// valid target.
    #[prost(message, optional, boxed, tag = "2")]
    pub over_target_indicator: ::core::option::Option<::prost::alloc::boxed::Box<Node>>,
    /// Action to invoke when the node is dropped over a target.
    #[prost(message, optional, tag = "3")]
    pub on_drop: ::core::option::Option<ClientAction>,
    /// User must drag the element through this horizontal distance in screen
    /// pixels before the UI responds. Useful to enable horizontal element
    /// dragging from a vertical scroll view.
    #[prost(message, optional, tag = "4")]
    pub horizontal_drag_start_distance: ::core::option::Option<u32>,
    /// If true, the original element is removed as part of this drag operation,
    /// causing it to visually appear as though the user is moving it instead of
    /// a placeholder.
    #[prost(bool, tag = "5")]
    pub remove_original: bool,
    /// Identifiers of children of this Draggable which should be hidden in the
    /// drag indicator element.
    #[prost(string, repeated, tag = "6")]
    pub hide_indicator_children: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DropTargetNode {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextFieldNode {
    /// Globally unique identifier for this text field, used to avoid
    /// overwriting user input. Cannot be the empty string.
    ///
    /// An initial value will only be set once on the TextField for a given
    /// identifier.
    #[prost(string, tag = "1")]
    pub global_identifier: ::prost::alloc::string::String,
    /// Text to initially display within the text field.
    #[prost(string, tag = "2")]
    pub initial_text: ::prost::alloc::string::String,
    /// Allow multiple lines of input text
    #[prost(bool, tag = "3")]
    pub multiline: bool,
    /// Whether the text can be edited
    #[prost(bool, tag = "4")]
    pub is_read_only: bool,
    /// Maximum number of characters for the field.
    #[prost(uint32, tag = "5")]
    pub max_length: u32,
    /// Set to true if the field is used to edit a password.
    #[prost(bool, tag = "6")]
    pub is_password_field: bool,
    /// Controls whether double clicking selects the word under the mouse
    /// pointer or not.
    #[prost(bool, tag = "7")]
    pub double_click_selects_word: bool,
    /// Controls whether triple clicking selects the entire line under the
    /// mouse pointer or not.
    #[prost(bool, tag = "8")]
    pub triple_click_selects_line: bool,
    /// The character used for masking in a password field.
    #[prost(string, tag = "9")]
    pub mask_character: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SliderNode {
    /// Value to display in the slider when first rendered.
    #[prost(float, tag = "1")]
    pub initial_value: f32,
    /// Label to display on this slider
    #[prost(string, tag = "2")]
    pub label: ::prost::alloc::string::String,
    /// If provided, the value of this slider will be read from and written
    /// to the float PlayerPreference with the provided key.
    #[prost(string, tag = "3")]
    pub preference_key: ::prost::alloc::string::String,
    //// Orientation of the slider. Defaults to horizontal.
    #[prost(enumeration = "SliderDirection", tag = "4")]
    pub direction: i32,
    /// The maximum value that the slider encodes.
    #[prost(float, tag = "5")]
    pub high_value: f32,
    /// This is the minimum value that the slider encodes.
    #[prost(float, tag = "6")]
    pub low_value: f32,
    /// This indicates whether or not this slider is inverted. For an inverted
    /// horizontal slider, high value is located to the left, low value is
    /// located to the right For an inverted vertical slider, high value is
    /// located to the bottom, low value is located to the top.
    #[prost(bool, tag = "7")]
    pub inverted: bool,
    /// Size used to increment or decrement the value when clicking within
    /// the slider.
    #[prost(float, tag = "8")]
    pub page_size: f32,
    /// The visibility of the optional field inside the slider control.
    #[prost(bool, tag = "9")]
    pub show_input_field: bool,
    #[prost(message, optional, tag = "10")]
    pub label_style: ::core::option::Option<FlexStyle>,
    #[prost(message, optional, tag = "11")]
    pub drag_container_style: ::core::option::Option<FlexStyle>,
    #[prost(message, optional, tag = "12")]
    pub tracker_style: ::core::option::Option<FlexStyle>,
    #[prost(message, optional, tag = "13")]
    pub dragger_style: ::core::option::Option<FlexStyle>,
    #[prost(message, optional, tag = "14")]
    pub dragger_border_style: ::core::option::Option<FlexStyle>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NodeType {
    #[prost(oneof = "node_type::NodeType", tags = "1, 2, 3, 4, 5, 6")]
    pub node_type: ::core::option::Option<node_type::NodeType>,
}
/// Nested message and enum types in `NodeType`.
pub mod node_type {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum NodeType {
        #[prost(message, tag = "1")]
        Text(super::Text),
        #[prost(message, tag = "2")]
        ScrollViewNode(super::ScrollViewNode),
        #[prost(message, tag = "3")]
        DraggableNode(::prost::alloc::boxed::Box<super::DraggableNode>),
        #[prost(message, tag = "4")]
        DropTargetNode(super::DropTargetNode),
        #[prost(message, tag = "5")]
        TextFieldNode(super::TextFieldNode),
        #[prost(message, tag = "6")]
        SliderNode(super::SliderNode),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventHandlers {
    #[prost(message, optional, tag = "1")]
    pub on_click: ::core::option::Option<ClientAction>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Node {
    /// Used to identify this node in the view hierarchy
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, optional, boxed, tag = "2")]
    pub node_type: ::core::option::Option<::prost::alloc::boxed::Box<NodeType>>,
    #[prost(message, repeated, tag = "3")]
    pub children: ::prost::alloc::vec::Vec<Node>,
    #[prost(message, optional, tag = "4")]
    pub event_handlers: ::core::option::Option<EventHandlers>,
    #[prost(message, optional, tag = "5")]
    pub style: ::core::option::Option<FlexStyle>,
    #[prost(message, optional, tag = "6")]
    pub hover_style: ::core::option::Option<FlexStyle>,
    #[prost(message, optional, tag = "7")]
    pub pressed_style: ::core::option::Option<FlexStyle>,
}
// ============================================================================
// Game Primitives
// ============================================================================

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerIdentifier {
    #[prost(oneof = "player_identifier::PlayerIdentifierType", tags = "1, 2, 3")]
    pub player_identifier_type: ::core::option::Option<player_identifier::PlayerIdentifierType>,
}
/// Nested message and enum types in `PlayerIdentifier`.
pub mod player_identifier {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PlayerIdentifierType {
        /// An identifier from Unity's social API: Social.localUser.id
        #[prost(string, tag = "1")]
        SocialIdentifier(::prost::alloc::string::String),
        /// An identifier for a device: SystemInfo.deviceUniqueIdentifier
        #[prost(string, tag = "2")]
        DeviceIdentifier(::prost::alloc::string::String),
        /// An opaque identifier specified on the server, e.g. for an AI player
        #[prost(bytes, tag = "3")]
        ServerIdentifier(::prost::alloc::vec::Vec<u8>),
    }
}
#[derive(Eq, Hash, Copy, Ord, PartialOrd, Clone, PartialEq, ::prost::Message)]
pub struct GameIdentifier {
    #[prost(uint64, tag = "1")]
    pub value: u64,
}
#[derive(Eq, Hash, Copy, Ord, PartialOrd, Clone, PartialEq, ::prost::Message)]
pub struct CardIdentifier {
    #[prost(enumeration = "PlayerSide", tag = "1")]
    pub side: i32,
    #[prost(uint32, tag = "2")]
    pub index: u32,
    /// Optionally, identifies a specific ability within a logical card which
    /// is represented by this displayed card.
    #[prost(message, optional, tag = "3")]
    pub ability_id: ::core::option::Option<u32>,
}
#[derive(Eq, Hash, Copy, Ord, PartialOrd, Clone, PartialEq, ::prost::Message)]
pub struct GameObjectIdentifier {
    #[prost(oneof = "game_object_identifier::Id", tags = "1, 2, 3, 4")]
    pub id: ::core::option::Option<game_object_identifier::Id>,
}
/// Nested message and enum types in `GameObjectIdentifier`.
pub mod game_object_identifier {
    #[derive(Eq, Hash, Copy, Ord, PartialOrd, Clone, PartialEq, ::prost::Oneof)]
    pub enum Id {
        #[prost(message, tag = "1")]
        CardId(super::CardIdentifier),
        #[prost(enumeration = "super::PlayerName", tag = "2")]
        Leader(i32),
        #[prost(enumeration = "super::PlayerName", tag = "3")]
        Deck(i32),
        #[prost(enumeration = "super::PlayerName", tag = "4")]
        DiscardPile(i32),
    }
}
// ============================================================================
// Game View
// ============================================================================

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CardIcon {
    /// Background for the icon.
    #[prost(message, optional, tag = "1")]
    pub background: ::core::option::Option<SpriteAddress>,
    /// Text to display on the icon.
    #[prost(message, optional, tag = "2")]
    pub text: ::core::option::Option<::prost::alloc::string::String>,
    /// Scale multiplier for the background image.
    #[prost(message, optional, tag = "3")]
    pub background_scale: ::core::option::Option<f32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CardIcons {
    #[prost(message, optional, tag = "1")]
    pub top_left_icon: ::core::option::Option<CardIcon>,
    #[prost(message, optional, tag = "2")]
    pub top_right_icon: ::core::option::Option<CardIcon>,
    #[prost(message, optional, tag = "3")]
    pub bottom_right_icon: ::core::option::Option<CardIcon>,
    #[prost(message, optional, tag = "4")]
    pub bottom_left_icon: ::core::option::Option<CardIcon>,
    #[prost(message, optional, tag = "5")]
    pub arena_icon: ::core::option::Option<CardIcon>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CardTitle {
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub text_color: ::core::option::Option<FlexColor>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RulesText {
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
}
/// Card has no targeting requirement
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NoTargeting {
    /// True if this card can currently be played
    #[prost(bool, tag = "1")]
    pub can_play: bool,
}
/// This card should prompt for a room to be played into.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayInRoom {
    /// The card can be played if at least one identifier is present here
    #[prost(enumeration = "RoomIdentifier", repeated, tag = "1")]
    pub valid_rooms: ::prost::alloc::vec::Vec<i32>,
}
/// The card should show an arrow to select a room to target
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ArrowTargetRoom {
    /// The card can be played if at least one identifier is present here
    #[prost(enumeration = "RoomIdentifier", repeated, tag = "1")]
    pub valid_rooms: ::prost::alloc::vec::Vec<i32>,
    /// Which arrow to show
    #[prost(enumeration = "TargetingArrow", tag = "2")]
    pub arrow: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CardTargeting {
    #[prost(oneof = "card_targeting::Targeting", tags = "1, 2, 3")]
    pub targeting: ::core::option::Option<card_targeting::Targeting>,
}
/// Nested message and enum types in `CardTargeting`.
pub mod card_targeting {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Targeting {
        #[prost(message, tag = "1")]
        NoTargeting(super::NoTargeting),
        #[prost(message, tag = "2")]
        PlayInRoom(super::PlayInRoom),
        #[prost(message, tag = "3")]
        ArrowTargetRoom(super::ArrowTargetRoom),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectPositionOffscreen {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectPositionRoom {
    #[prost(enumeration = "RoomIdentifier", tag = "1")]
    pub room_id: i32,
    #[prost(enumeration = "ClientRoomLocation", tag = "2")]
    pub room_location: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectPositionItem {
    #[prost(enumeration = "ClientItemLocation", tag = "1")]
    pub item_location: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectPositionStaging {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectPositionHand {
    #[prost(enumeration = "PlayerName", tag = "1")]
    pub owner: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectPositionDeck {
    #[prost(enumeration = "PlayerName", tag = "1")]
    pub owner: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectPositionDeckContainer {
    #[prost(enumeration = "PlayerName", tag = "1")]
    pub owner: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectPositionDiscardPile {
    #[prost(enumeration = "PlayerName", tag = "1")]
    pub owner: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectPositionDiscardPileContainer {
    #[prost(enumeration = "PlayerName", tag = "1")]
    pub owner: i32,
}
/// Large display of cards *while* the score animation is playing. After the
/// score animation finishes, scored cards move to 'Leader' position.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectPositionScoreAnimation {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectPositionRaid {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectPositionBrowser {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectPositionLeader {
    #[prost(enumeration = "PlayerName", tag = "1")]
    pub owner: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectPositionLeaderContainer {
    #[prost(enumeration = "PlayerName", tag = "1")]
    pub owner: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectPositionRewardChest {}
//// An object position which represents moving into a given card.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectPositionIntoCard {
    #[prost(message, optional, tag = "1")]
    pub card_id: ::core::option::Option<CardIdentifier>,
}
//// An object position for newly-revealed cards, appears above other content
//// like the staging area.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectPositionRevealedCards {
    #[prost(enumeration = "RevealedCardsBrowserSize", tag = "1")]
    pub size: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectPosition {
    /// A key by which to sort this object -- objects with higher sorting keys
    /// should be displayed 'on top of' or 'in front of' objects with lower
    /// sorting keys.
    ///
    /// NOTE: Despite the fact that Unity uses the 'int' type for this in C#,
    /// they actually store these as 16-bit signed integers, and your code
    /// silently breaks if you use a number over 32,767!
    #[prost(uint32, tag = "1")]
    pub sorting_key: u32,
    /// An additional key, can be used to break ties in `sorting_key`
    #[prost(uint32, tag = "2")]
    pub sorting_subkey: u32,
    #[prost(
        oneof = "object_position::Position",
        tags = "3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14, 15, 16, 17, 18"
    )]
    pub position: ::core::option::Option<object_position::Position>,
}
/// Nested message and enum types in `ObjectPosition`.
pub mod object_position {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Position {
        #[prost(message, tag = "3")]
        Offscreen(super::ObjectPositionOffscreen),
        #[prost(message, tag = "4")]
        Room(super::ObjectPositionRoom),
        #[prost(message, tag = "5")]
        Item(super::ObjectPositionItem),
        #[prost(message, tag = "6")]
        Staging(super::ObjectPositionStaging),
        #[prost(message, tag = "7")]
        Hand(super::ObjectPositionHand),
        #[prost(message, tag = "8")]
        Deck(super::ObjectPositionDeck),
        #[prost(message, tag = "9")]
        DeckContainer(super::ObjectPositionDeckContainer),
        #[prost(message, tag = "10")]
        DiscardPile(super::ObjectPositionDiscardPile),
        #[prost(message, tag = "11")]
        DiscardPileContainer(super::ObjectPositionDiscardPileContainer),
        #[prost(message, tag = "13")]
        Raid(super::ObjectPositionRaid),
        #[prost(message, tag = "14")]
        Browser(super::ObjectPositionBrowser),
        #[prost(message, tag = "15")]
        Leader(super::ObjectPositionLeader),
        #[prost(message, tag = "16")]
        LeaderContainer(super::ObjectPositionLeaderContainer),
        #[prost(message, tag = "17")]
        IntoCard(super::ObjectPositionIntoCard),
        #[prost(message, tag = "18")]
        Revealed(super::ObjectPositionRevealedCards),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RevealedCardView {
    #[prost(message, optional, tag = "1")]
    pub card_frame: ::core::option::Option<SpriteAddress>,
    #[prost(message, optional, tag = "2")]
    pub title_background: ::core::option::Option<SpriteAddress>,
    #[prost(message, optional, tag = "3")]
    pub jewel: ::core::option::Option<SpriteAddress>,
    #[prost(message, optional, tag = "4")]
    pub image: ::core::option::Option<SpriteAddress>,
    #[prost(message, optional, tag = "5")]
    pub title: ::core::option::Option<CardTitle>,
    #[prost(message, optional, tag = "6")]
    pub rules_text: ::core::option::Option<RulesText>,
    /// Custom targeting behavior for a card. If unspecified, no targeting UI
    /// is shown.
    #[prost(message, optional, tag = "7")]
    pub targeting: ::core::option::Option<CardTargeting>,
    /// Where to move a played card. Information from 'targeting' will be
    /// incorporated to fill this in, e.g. if a room is targeted and
    /// ObjectPositionRoom is selected here with no RoomId, the targeted room
    /// is used.
    #[prost(message, optional, tag = "8")]
    pub on_release_position: ::core::option::Option<ObjectPosition>,
    /// Additional interface element rendered to the side of the card during an
    /// info zoom.
    #[prost(message, optional, tag = "9")]
    pub supplemental_info: ::core::option::Option<Node>,
    /// Content to display behind the main image
    #[prost(message, optional, tag = "10")]
    pub image_background: ::core::option::Option<SpriteAddress>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CardView {
    #[prost(message, optional, tag = "1")]
    pub card_id: ::core::option::Option<CardIdentifier>,
    /// Where is this card located in the game?
    #[prost(message, optional, tag = "2")]
    pub card_position: ::core::option::Option<ObjectPosition>,
    /// Which prefab to use for this card, controls the overall appearance
    #[prost(enumeration = "CardPrefab", tag = "3")]
    pub prefab: i32,
    /// Whether the viewer (current player) is able to see the front of this
    /// card.
    #[prost(bool, tag = "4")]
    pub revealed_to_viewer: bool,
    /// Whether the card is in the 'face up' state.
    #[prost(bool, tag = "5")]
    pub is_face_up: bool,
    #[prost(message, optional, tag = "6")]
    pub card_icons: ::core::option::Option<CardIcons>,
    /// Frame shown on arena card when face-up
    #[prost(message, optional, tag = "7")]
    pub arena_frame: ::core::option::Option<SpriteAddress>,
    /// Frame shown on arena card when face-down
    #[prost(message, optional, tag = "8")]
    pub face_down_arena_frame: ::core::option::Option<SpriteAddress>,
    /// Used to e.g. determine which card back to display for this card.
    #[prost(enumeration = "PlayerName", tag = "9")]
    pub owning_player: i32,
    /// Card information which is only present on revealed cards.
    #[prost(message, optional, tag = "10")]
    pub revealed_card: ::core::option::Option<RevealedCardView>,
    /// Optionally, a position at which to create this card.
    ///
    /// If this card does not already exist, it will be created at this position
    /// before being animated to its 'card_position'.
    #[prost(message, optional, tag = "11")]
    pub create_position: ::core::option::Option<ObjectPosition>,
    /// Optionally, a position at which to destroy this card.
    ///
    /// If provided, the card will be animated to this position before being
    /// destroyed.
    #[prost(message, optional, tag = "12")]
    pub destroy_position: ::core::option::Option<ObjectPosition>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerInfo {
    #[prost(message, optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "2")]
    pub arena_portrait: ::core::option::Option<SpriteAddress>,
    /// Rooms which this player can currently visit (raid/level up)
    #[prost(enumeration = "RoomIdentifier", repeated, tag = "3")]
    pub valid_rooms_to_visit: ::prost::alloc::vec::Vec<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ManaView {
    #[prost(uint32, tag = "1")]
    pub base_mana: u32,
    /// Additional mana with custom use restrictions.
    #[prost(uint32, tag = "2")]
    pub bonus_mana: u32,
    /// Can the viewer currently take the 'gain mana' action on this mana
    /// display?
    #[prost(bool, tag = "3")]
    pub can_take_gain_mana_action: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScoreView {
    #[prost(uint32, tag = "1")]
    pub score: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActionTrackerView {
    #[prost(uint32, tag = "1")]
    pub available_action_count: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeckView {
    /// How many cards are in this deck?
    #[prost(uint32, tag = "1")]
    pub card_count: u32,
    /// Card back asset to use for this player's cards.
    #[prost(message, optional, tag = "2")]
    pub card_back: ::core::option::Option<SpriteAddress>,
    /// Can the viewer currently take the 'draw card' action on this deck?
    #[prost(bool, tag = "3")]
    pub can_take_draw_card_action: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayerView {
    #[prost(enumeration = "PlayerSide", tag = "1")]
    pub side: i32,
    #[prost(message, optional, tag = "2")]
    pub player_info: ::core::option::Option<PlayerInfo>,
    #[prost(message, optional, tag = "3")]
    pub score: ::core::option::Option<ScoreView>,
    #[prost(message, optional, tag = "4")]
    pub mana: ::core::option::Option<ManaView>,
    #[prost(message, optional, tag = "5")]
    pub action_tracker: ::core::option::Option<ActionTrackerView>,
    #[prost(message, optional, tag = "6")]
    pub deck_view: ::core::option::Option<DeckView>,
    /// Whether this player is currently able to take some game action
    #[prost(bool, tag = "7")]
    pub can_take_action: bool,
}
/// Positions of non-Card game objects.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GameObjectPositions {
    #[prost(message, optional, tag = "1")]
    pub user_deck: ::core::option::Option<ObjectPosition>,
    #[prost(message, optional, tag = "2")]
    pub opponent_deck: ::core::option::Option<ObjectPosition>,
    #[prost(message, optional, tag = "3")]
    pub user_leader: ::core::option::Option<ObjectPosition>,
    #[prost(message, optional, tag = "4")]
    pub opponent_leader: ::core::option::Option<ObjectPosition>,
    #[prost(message, optional, tag = "5")]
    pub user_discard: ::core::option::Option<ObjectPosition>,
    #[prost(message, optional, tag = "6")]
    pub opponent_discard: ::core::option::Option<ObjectPosition>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShowArrowBubble {
    /// Text to show.
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    /// How long the user needs to be idle for before displaying this effect. If
    /// not specified, will show immediately.
    #[prost(message, optional, tag = "2")]
    pub idle_timer: ::core::option::Option<TimeValue>,
    /// Time before the popup should be hidden automatically. If not specified,
    /// will remain permanently.
    #[prost(message, optional, tag = "3")]
    pub hide_time: ::core::option::Option<TimeValue>,
    /// Background color. Defaults to white.
    #[prost(message, optional, tag = "4")]
    pub color: ::core::option::Option<FlexColor>,
    /// Size of displayed text in Unity font units. Defaults to 3.0.
    #[prost(message, optional, tag = "5")]
    pub font_size: ::core::option::Option<f32>,
    /// Color of text. Defaults to black.
    #[prost(message, optional, tag = "6")]
    pub font_color: ::core::option::Option<FlexColor>,
    /// Multiplier for size of arrow buble. Defaults to 1.0.
    #[prost(message, optional, tag = "7")]
    pub scale: ::core::option::Option<f32>,
    #[prost(enumeration = "ArrowBubbleCorner", tag = "8")]
    pub arrow_corner: i32,
    /// Where to display the arrow bubble
    #[prost(oneof = "show_arrow_bubble::ArrowBubbleAnchor", tags = "9, 10, 11, 12")]
    pub arrow_bubble_anchor: ::core::option::Option<show_arrow_bubble::ArrowBubbleAnchor>,
}
/// Nested message and enum types in `ShowArrowBubble`.
pub mod show_arrow_bubble {
    /// Where to display the arrow bubble
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ArrowBubbleAnchor {
        /// Arrow pointing to a player
        #[prost(enumeration = "super::PlayerName", tag = "9")]
        Player(i32),
        /// Arrow pointing to a room
        #[prost(enumeration = "super::RoomIdentifier", tag = "10")]
        Room(i32),
        /// Arrow pointing to a player's deck
        #[prost(enumeration = "super::PlayerName", tag = "11")]
        PlayerDeck(i32),
        /// Arrow pointing to a player's mana
        #[prost(enumeration = "super::PlayerName", tag = "12")]
        PlayerMana(i32),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShowToast {
    /// Content to show inside the toast
    #[prost(message, optional, tag = "1")]
    pub node: ::core::option::Option<Node>,
    /// How long the user needs to be idle for before displaying this effect. If
    /// not specified, will show immediately.
    #[prost(message, optional, tag = "2")]
    pub idle_timer: ::core::option::Option<TimeValue>,
    /// Time before the popup should be hidden automatically. If not specified,
    /// will remain permanently.
    #[prost(message, optional, tag = "3")]
    pub hide_time: ::core::option::Option<TimeValue>,
}
/// Displays a tutorial UI element to the user when the user is idle for a fixed
/// time period.
///
/// Taking any game action resets the timer, and the timer doesn't start while
/// network requests are pending.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TutorialEffect {
    #[prost(oneof = "tutorial_effect::TutorialEffectType", tags = "1, 2")]
    pub tutorial_effect_type: ::core::option::Option<tutorial_effect::TutorialEffectType>,
}
/// Nested message and enum types in `TutorialEffect`.
pub mod tutorial_effect {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TutorialEffectType {
        /// Arrow bubble representing a tooltip or text spoken by a player
        /// in the game
        #[prost(message, tag = "1")]
        ArrowBubble(super::ShowArrowBubble),
        /// Pops up a message to provide help context for the user. Only one
        /// toast can be displayed at a time.
        #[prost(message, tag = "2")]
        ShowToast(super::ShowToast),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GameView {
    #[prost(message, optional, tag = "1")]
    pub user: ::core::option::Option<PlayerView>,
    #[prost(message, optional, tag = "2")]
    pub opponent: ::core::option::Option<PlayerView>,
    /// Updated values for the cards in this game. Any cards which have changed
    /// position should be moved to their new positions in parallel. Cards which
    /// do not exist in this list must be destroyed.
    #[prost(message, repeated, tag = "3")]
    pub cards: ::prost::alloc::vec::Vec<CardView>,
    /// Whether a raid is currently active. If true, the raid overlay will be
    /// displayed, the raid music will be played, etc.
    #[prost(bool, tag = "4")]
    pub raid_active: bool,
    /// Positions of non-Card game objects.
    #[prost(message, optional, tag = "5")]
    pub game_object_positions: ::core::option::Option<GameObjectPositions>,
    /// Controls for game actions such as interface prompts
    #[prost(message, optional, tag = "6")]
    pub main_controls: ::core::option::Option<InterfaceMainControls>,
    /// Tutorial UI elements
    #[prost(message, repeated, tag = "7")]
    pub tutorial_effects: ::prost::alloc::vec::Vec<TutorialEffect>,
}
// ============================================================================
// Actions
// ============================================================================

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StandardAction {
    /// Opaque payload to send to the server when invoked.
    #[prost(bytes = "vec", tag = "1")]
    pub payload: ::prost::alloc::vec::Vec<u8>,
    /// Immediate optimistic mutations to state for this action.
    #[prost(message, optional, tag = "2")]
    pub update: ::core::option::Option<CommandList>,
    /// User interface fields to read values from.
    ///
    /// If this map is not empty, the client will look for fields in the UI with
    /// names matching the keys of this map and set the contents of those fields
    /// as the values of this map when sending the action payload to the server.
    /// By convention, field names should be mapped to the empty string when
    /// initially returned from the server.
    #[prost(map = "string, string", tag = "3")]
    pub request_fields:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
}
/// Spend an action to gain 1 mana.
/// Optimistic: Mana is added immediately.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GainManaAction {}
/// Spend an action to draw a card.
/// Optimistic: Face-down card animates to reveal area.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DrawCardAction {}
/// Spend an action to level up a room.
/// Optimistic: Room visit animation plays
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LevelUpRoomAction {
    #[prost(enumeration = "RoomIdentifier", tag = "1")]
    pub room_id: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CardTarget {
    #[prost(oneof = "card_target::CardTarget", tags = "1")]
    pub card_target: ::core::option::Option<card_target::CardTarget>,
}
/// Nested message and enum types in `CardTarget`.
pub mod card_target {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum CardTarget {
        #[prost(enumeration = "super::RoomIdentifier", tag = "1")]
        RoomId(i32),
    }
}
/// Spend an action to play a card from hand.
/// Optimistic:
///   - Card animates to its 'on_release' position. If the RoomIdentifier is
///     unspecified for a room position, the targeted room is used.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayCardAction {
    #[prost(message, optional, tag = "1")]
    pub card_id: ::core::option::Option<CardIdentifier>,
    #[prost(message, optional, tag = "2")]
    pub target: ::core::option::Option<CardTarget>,
}
/// Spend an action to initiate a raid on one of the overlord's rooms
/// Optimistic: Room visit animation plays
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InitiateRaidAction {
    #[prost(enumeration = "RoomIdentifier", tag = "1")]
    pub room_id: i32,
}
/// Fetch the contents of a given interface panel.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchPanelAction {
    #[prost(message, optional, tag = "1")]
    pub panel_address: ::core::option::Option<InterfacePanelAddress>,
}
/// Spend an action point with no other effect, typically used for
/// tests
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpendActionPointAction {}
/// Possible game actions taken by the user.
///
/// Actions have an associated 'optimistic' behavior to display while waiting
/// for a server response. The client should not send multiple actions at the
/// same time -- interaction should be disabled while an action is pending.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientAction {
    #[prost(oneof = "client_action::Action", tags = "1, 2, 3, 4, 5, 6, 7, 8")]
    pub action: ::core::option::Option<client_action::Action>,
}
/// Nested message and enum types in `ClientAction`.
pub mod client_action {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Action {
        #[prost(message, tag = "1")]
        StandardAction(super::StandardAction),
        #[prost(message, tag = "2")]
        FetchPanel(super::FetchPanelAction),
        #[prost(message, tag = "3")]
        GainMana(super::GainManaAction),
        #[prost(message, tag = "4")]
        DrawCard(super::DrawCardAction),
        #[prost(message, tag = "5")]
        PlayCard(super::PlayCardAction),
        #[prost(message, tag = "6")]
        LevelUpRoom(super::LevelUpRoomAction),
        #[prost(message, tag = "7")]
        InitiateRaid(super::InitiateRaidAction),
        #[prost(message, tag = "8")]
        SpendActionPoint(super::SpendActionPointAction),
    }
}
/// Initiate a play session and download the current state for the
/// provided player.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConnectRequest {
    /// User making this request.
    #[prost(message, optional, tag = "1")]
    pub player_id: ::core::option::Option<PlayerIdentifier>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GameRequest {
    #[prost(message, optional, tag = "1")]
    pub action: ::core::option::Option<ClientAction>,
    /// Identifies the user making this request. At some point I'm going to
    /// figure out how to set up authentication, but currently we operate on
    /// the honor system :)
    #[prost(message, optional, tag = "2")]
    pub player_id: ::core::option::Option<PlayerIdentifier>,
    /// Interface panels which were open at the time of the action, to be
    /// updated.
    #[prost(message, repeated, tag = "3")]
    pub open_panels: ::prost::alloc::vec::Vec<InterfacePanelAddress>,
}
// ============================================================================
// Commands
// ============================================================================

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DebugLogCommand {
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
}
/// Run a series of command lists simultaneously. Warning: applying multiple
/// commands to the same game object will have unpredictable results.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunInParallelCommand {
    #[prost(message, repeated, tag = "1")]
    pub commands: ::prost::alloc::vec::Vec<CommandList>,
}
/// Wait before executing the next command in sequence.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelayCommand {
    #[prost(message, optional, tag = "1")]
    pub duration: ::core::option::Option<TimeValue>,
}
/// Identifies an InterfacePanel.
#[derive(Eq, Hash, Clone, PartialEq, ::prost::Message)]
pub struct InterfacePanelAddress {
    #[prost(bytes = "vec", tag = "1")]
    pub serialized: ::prost::alloc::vec::Vec<u8>,
}
/// A 'panel' is an independently addressable block of UI. The contents
/// of each known panel are cached and can then be opened immediately
/// by the client, without waiting for a server response.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InterfacePanel {
    #[prost(message, optional, tag = "1")]
    pub address: ::core::option::Option<InterfacePanelAddress>,
    #[prost(message, optional, tag = "2")]
    pub node: ::core::option::Option<Node>,
    /// Optionally, a global screen overlay which should be displayed while
    /// this panel is open, replacing the global overlay provided via
    /// RenderScreenOverlayCommand.
    #[prost(message, optional, tag = "3")]
    pub screen_overlay: ::core::option::Option<Node>,
}
/// Requests that a specific corner of a Node be anchored to a specific
/// corner of a card.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CardAnchor {
    #[prost(enumeration = "AnchorCorner", tag = "1")]
    pub node_corner: i32,
    #[prost(enumeration = "AnchorCorner", tag = "2")]
    pub card_corner: i32,
}
/// Render an interface element attached to a specific card.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CardAnchorNode {
    #[prost(message, optional, tag = "1")]
    pub card_id: ::core::option::Option<CardIdentifier>,
    #[prost(message, optional, tag = "2")]
    pub node: ::core::option::Option<Node>,
    /// Used to set the absolute position inset of 'node' to match corners of
    /// the identified card. Later anchors in this list overwrite earlier
    /// anchors in the case of a conflict.
    #[prost(message, repeated, tag = "3")]
    pub anchors: ::prost::alloc::vec::Vec<CardAnchor>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InterfaceMainControls {
    /// Main controls area
    #[prost(message, optional, tag = "1")]
    pub node: ::core::option::Option<Node>,
    /// Controls for specific cards
    #[prost(message, repeated, tag = "3")]
    pub card_anchor_nodes: ::prost::alloc::vec::Vec<CardAnchorNode>,
}
/// Updates the contents of one or more user interface panels
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatePanelsCommand {
    /// List of panels to update.
    #[prost(message, repeated, tag = "1")]
    pub panels: ::prost::alloc::vec::Vec<InterfacePanel>,
}
/// Open a panel and display the provided loading state while it is
/// being fetched
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressWithLoadingState {
    #[prost(message, optional, tag = "1")]
    pub open_panel: ::core::option::Option<InterfacePanelAddress>,
    /// Content to display if this panel is not already cached
    #[prost(message, optional, tag = "2")]
    pub loading_state: ::core::option::Option<Node>,
}
/// Options for transitioning to a new panel.
///
/// Will log an error if 'open' is not available and no loading state is
/// provided, or if the loading state is not available.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PanelTransitionOptions {
    /// New panel to open
    #[prost(message, optional, tag = "1")]
    pub open: ::core::option::Option<InterfacePanelAddress>,
    /// Previous panel to close, if any
    #[prost(message, optional, tag = "2")]
    pub close: ::core::option::Option<InterfacePanelAddress>,
    /// Panel to display if 'open' has not yet been fetched
    #[prost(message, optional, tag = "3")]
    pub loading: ::core::option::Option<InterfacePanelAddress>,
    /// If true, do not attempt to fetch the 'open' panel, just wait for
    /// it to be returned
    #[prost(bool, tag = "4")]
    pub do_not_fetch: bool,
    /// If true, displays a loading animation on the 'close' screen while
    /// fetching the 'open' screen, then transitions once it is loaded.
    #[prost(bool, tag = "5")]
    pub wait_to_load: bool,
}
/// Requests to open or close the given interface panel.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TogglePanelCommand {
    #[prost(oneof = "toggle_panel_command::ToggleCommand", tags = "1, 2, 3, 4, 5")]
    pub toggle_command: ::core::option::Option<toggle_panel_command::ToggleCommand>,
}
/// Nested message and enum types in `TogglePanelCommand`.
pub mod toggle_panel_command {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ToggleCommand {
        /// Open a new panel with transition options.
        #[prost(message, tag = "1")]
        Transition(super::PanelTransitionOptions),
        /// Opens a new bottom sheet with the indicated panel.
        ///
        /// Closes any existing bottom sheet.
        #[prost(message, tag = "2")]
        OpenBottomSheetAddress(super::InterfacePanelAddress),
        /// Closes the currently-open bottom sheet.
        #[prost(message, tag = "3")]
        CloseBottomSheet(()),
        /// Pushes the indicated panel as a new bottom sheet page.
        ///
        /// If no bottom sheet is currently open, the behavior is identical to
        /// 'open_bottom_sheet'.
        #[prost(message, tag = "4")]
        PushBottomSheetAddress(super::InterfacePanelAddress),
        /// Pops the currently visible bottom sheet page and displays the
        /// indicated panel as the *new* sheet content.
        ///
        /// If no bottom sheet is currently open, the behavior is identical to
        /// 'open_bottom_sheet'.
        #[prost(message, tag = "5")]
        PopToBottomSheetAddress(super::InterfacePanelAddress),
    }
}
/// Updates the current GameView state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateGameViewCommand {
    #[prost(message, optional, tag = "1")]
    pub game: ::core::option::Option<GameView>,
    /// Whether this update should be animated
    #[prost(bool, tag = "2")]
    pub animate: bool,
}
/// Animates 'initiator' moving to a room and plays a standard particle effect
/// based on the visit type.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VisitRoomCommand {
    #[prost(enumeration = "PlayerName", tag = "1")]
    pub initiator: i32,
    #[prost(enumeration = "RoomIdentifier", tag = "2")]
    pub room_id: i32,
    #[prost(enumeration = "RoomVisitType", tag = "3")]
    pub visit_type: i32,
}
/// Creates a new token card.
///
/// This command is typically used to create short-lived 'token' cards to
/// represent things like abilities firing, but this isn't specifically
/// required. If a matching CardIdentifier already exists, that card will be
/// updated instead.
///
/// Note that the created card will always be deleted by the next
/// UpdateGameViewCommand if its ID is not present in that update.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTokenCardCommand {
    #[prost(message, optional, tag = "1")]
    pub card: ::core::option::Option<CardView>,
    /// Whether this update should be animated
    #[prost(bool, tag = "2")]
    pub animate: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GameObjectMove {
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<GameObjectIdentifier>,
    #[prost(message, optional, tag = "2")]
    pub position: ::core::option::Option<ObjectPosition>,
}
/// Move a list of game objects to new positions, in parallel
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveGameObjectsCommand {
    #[prost(message, repeated, tag = "1")]
    pub moves: ::prost::alloc::vec::Vec<GameObjectMove>,
    #[prost(bool, tag = "2")]
    pub disable_animation: bool,
    /// A delay once the cards reach their destination
    #[prost(message, optional, tag = "3")]
    pub delay: ::core::option::Option<TimeValue>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlaySoundCommand {
    #[prost(message, optional, tag = "1")]
    pub sound: ::core::option::Option<AudioClipAddress>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetMusicCommand {
    #[prost(enumeration = "MusicState", tag = "1")]
    pub music_state: i32,
}
/// Fire a projectile from one game object at another.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FireProjectileCommand {
    #[prost(message, optional, tag = "1")]
    pub source_id: ::core::option::Option<GameObjectIdentifier>,
    #[prost(message, optional, tag = "2")]
    pub target_id: ::core::option::Option<GameObjectIdentifier>,
    /// Projectile to fire from the 'source_id' card to 'target_id'
    #[prost(message, optional, tag = "3")]
    pub projectile: ::core::option::Option<ProjectileAddress>,
    /// How long the projectile should take to hit its target.
    #[prost(message, optional, tag = "4")]
    pub travel_duration: ::core::option::Option<TimeValue>,
    #[prost(message, optional, tag = "5")]
    pub fire_sound: ::core::option::Option<AudioClipAddress>,
    #[prost(message, optional, tag = "6")]
    pub impact_sound: ::core::option::Option<AudioClipAddress>,
    /// Additional effect to display on the target on hit.
    #[prost(message, optional, tag = "7")]
    pub additional_hit: ::core::option::Option<EffectAddress>,
    /// Delay before showing the additional hit. If provided, the original
    /// projectile Hit effect will be hidden before showing the new hit effect.
    #[prost(message, optional, tag = "8")]
    pub additional_hit_delay: ::core::option::Option<TimeValue>,
    /// During to wait for the project's impact effect before continuing
    #[prost(message, optional, tag = "9")]
    pub wait_duration: ::core::option::Option<TimeValue>,
    /// If true, the target will be hidden after being hit during the
    /// 'wait_duration' and before jumping to 'jump_to_position'.
    #[prost(bool, tag = "10")]
    pub hide_on_hit: bool,
    /// Position for the target to jump to after being hit.
    #[prost(message, optional, tag = "11")]
    pub jump_to_position: ::core::option::Option<ObjectPosition>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayEffectPosition {
    #[prost(oneof = "play_effect_position::EffectPosition", tags = "1")]
    pub effect_position: ::core::option::Option<play_effect_position::EffectPosition>,
}
/// Nested message and enum types in `PlayEffectPosition`.
pub mod play_effect_position {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum EffectPosition {
        #[prost(message, tag = "1")]
        GameObject(super::GameObjectIdentifier),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlayEffectCommand {
    #[prost(message, optional, tag = "1")]
    pub effect: ::core::option::Option<EffectAddress>,
    #[prost(message, optional, tag = "2")]
    pub position: ::core::option::Option<PlayEffectPosition>,
    #[prost(message, optional, tag = "3")]
    pub scale: ::core::option::Option<f32>,
    /// How long to wait before continuing.
    #[prost(message, optional, tag = "4")]
    pub duration: ::core::option::Option<TimeValue>,
    #[prost(message, optional, tag = "5")]
    pub sound: ::core::option::Option<AudioClipAddress>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisplayGameMessageCommand {
    #[prost(enumeration = "GameMessageType", tag = "1")]
    pub message_type: i32,
}
/// Used to hide and show all game UI elements.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetGameObjectsEnabledCommand {
    #[prost(bool, tag = "1")]
    pub game_objects_enabled: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisplayRewardsCommand {
    #[prost(message, repeated, tag = "1")]
    pub rewards: ::prost::alloc::vec::Vec<CardView>,
}
/// Loads a named Unity scene
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoadSceneCommand {
    #[prost(string, tag = "1")]
    pub scene_name: ::prost::alloc::string::String,
    #[prost(enumeration = "SceneLoadMode", tag = "2")]
    pub mode: i32,
    /// If true, skip loading this scene if it matches the currently-loaded
    /// main scene.
    #[prost(bool, tag = "3")]
    pub skip_if_current: bool,
}
/// Sets a client-side boolean player preference
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetBooleanPreference {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub value: bool,
}
/// Logs a client message
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogMessage {
    #[prost(string, tag = "1")]
    pub text: ::prost::alloc::string::String,
    #[prost(enumeration = "LogMessageLevel", tag = "2")]
    pub level: i32,
}
/// Activates client-side debugging functionality
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientDebugCommand {
    #[prost(oneof = "client_debug_command::DebugCommand", tags = "1, 2, 3, 4")]
    pub debug_command: ::core::option::Option<client_debug_command::DebugCommand>,
}
/// Nested message and enum types in `ClientDebugCommand`.
pub mod client_debug_command {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum DebugCommand {
        #[prost(message, tag = "1")]
        ShowLogs(()),
        #[prost(message, tag = "2")]
        InvokeAction(super::ClientAction),
        #[prost(message, tag = "3")]
        LogMessage(super::LogMessage),
        #[prost(message, tag = "4")]
        SetBooleanPreference(super::SetBooleanPreference),
    }
}
/// Position of a tile on the world map
///
/// We use offset hex coordinates with the "Pointy Top - Odd Rows Shifted
/// Right" convention, with values increasing moving up and right.
///
/// ```text
/// 
///       /  \    / \
///     /     \ /     \
///    |  0,2  |  1,2  |
///    |       |       |
///   / \     / \     / \
/// /     \ /     \ /     \
/// |  0,1  |  1,1  |  2,1  |
/// |       |       |       |
/// \     / \     / \     /
///   \ /     \ /     \ /
///    |  0,0  |  1,0  |
///    |       |       |
///     \     / \     /
///       \ /     \ /
/// ```
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MapPosition {
    #[prost(int32, tag = "1")]
    pub x: i32,
    #[prost(int32, tag = "2")]
    pub y: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorldMapSprite {
    /// Addressable asset path of sprite to display on the hex grid
    #[prost(message, optional, tag = "1")]
    pub sprite_address: ::core::option::Option<SpriteAddress>,
    /// Color tint for the provided sprite.
    #[prost(message, optional, tag = "2")]
    pub color: ::core::option::Option<FlexColor>,
    /// Controls the position of the tile image. Note that tiles by default are
    /// anchored at (0,-0.64), meaning they're shifted to screen bottom.
    #[prost(message, optional, tag = "3")]
    pub anchor_offset: ::core::option::Option<FlexVector3>,
    /// Scale transformation to apply to the image.
    #[prost(message, optional, tag = "4")]
    pub scale: ::core::option::Option<FlexVector3>,
}
/// Represents the contents of a world map tile.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorldMapTile {
    /// Images to display on this tile. Will be rendered in z-index order, with
    /// later sprites appearing on top of earlier ones. Sprites always display
    /// below the player character layer.
    #[prost(message, repeated, tag = "1")]
    pub sprites: ::prost::alloc::vec::Vec<WorldMapSprite>,
    /// Tile position.
    #[prost(message, optional, tag = "2")]
    pub position: ::core::option::Option<MapPosition>,
    /// Action to invoke when this tile is visited by the player.
    #[prost(message, optional, tag = "3")]
    pub on_visit: ::core::option::Option<ClientAction>,
    /// How can the player character navigate through this tile?
    #[prost(enumeration = "MapTileType", tag = "4")]
    pub tile_type: i32,
}
/// Updates the world map tilemap. Only valid in the 'World' scene.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateWorldMapCommand {
    #[prost(message, repeated, tag = "1")]
    pub tiles: ::prost::alloc::vec::Vec<WorldMapTile>,
}
/// Displays a UI element on top of all other elements. This is typically used
/// to render chrome, e.g. buttons related to global navigation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenderScreenOverlayCommand {
    #[prost(message, optional, tag = "1")]
    pub node: ::core::option::Option<Node>,
}
/// A method for unqiuely identifying a single user interface element
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ElementSelector {
    #[prost(oneof = "element_selector::Selector", tags = "1, 2, 3")]
    pub selector: ::core::option::Option<element_selector::Selector>,
}
/// Nested message and enum types in `ElementSelector`.
pub mod element_selector {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Selector {
        /// Identify an element by name
        #[prost(string, tag = "1")]
        ElementName(::prost::alloc::string::String),
        /// The element currently being dragged
        #[prost(message, tag = "2")]
        DragIndicator(()),
        /// A synthetic element created via an operation such as
        /// 'CreateTargetAtChildIndex'.
        #[prost(string, tag = "3")]
        TargetElement(::prost::alloc::string::String),
    }
}
/// Describes how to animate an element change
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ElementAnimation {
    /// Duration over which to animate the change.
    #[prost(message, optional, tag = "1")]
    pub duration: ::core::option::Option<TimeValue>,
    /// Easing curve to use for the element animation.
    #[prost(enumeration = "EasingMode", tag = "2")]
    pub ease: i32,
}
/// Animates the element to match the position of another element
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnimateToPosition {
    #[prost(message, optional, tag = "1")]
    pub destination: ::core::option::Option<ElementSelector>,
    #[prost(message, optional, tag = "2")]
    pub animation: ::core::option::Option<ElementAnimation>,
    /// If false, the Y coordinate of the target positon is offset by 1/2
    /// the source element's height.
    #[prost(bool, tag = "3")]
    pub disable_height_half_offset: bool,
    /// If false, the X coordinate of the target positon is offset by 1/2
    /// the source element's width
    #[prost(bool, tag = "4")]
    pub disable_width_half_offset: bool,
}
/// Creates a cloned invisible 'target' element at a given child index position
/// of a parent element. The target starts at 1x1 size and animates its width
/// and height to match the size of the source element. After reaching full
/// size, it becomes visible.
///
/// The target can be retrieved via the 'target' element selector using the
/// provided target_name. These element names only need to be unique among
/// other targets, it is idiomatic to use the same name as the source element.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateTargetAtChildIndex {
    #[prost(message, optional, tag = "1")]
    pub parent: ::core::option::Option<ElementSelector>,
    #[prost(uint32, tag = "2")]
    pub index: u32,
    #[prost(string, tag = "3")]
    pub target_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub animation: ::core::option::Option<ElementAnimation>,
}
/// Animates a style property of an element to a new value
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AnimateElementStyle {
    #[prost(message, optional, tag = "1")]
    pub animation: ::core::option::Option<ElementAnimation>,
    #[prost(oneof = "animate_element_style::Property", tags = "2, 3, 4, 5")]
    pub property: ::core::option::Option<animate_element_style::Property>,
}
/// Nested message and enum types in `AnimateElementStyle`.
pub mod animate_element_style {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Property {
        #[prost(float, tag = "2")]
        Opacity(f32),
        #[prost(float, tag = "3")]
        Width(f32),
        #[prost(float, tag = "4")]
        Height(f32),
        #[prost(message, tag = "5")]
        Scale(super::FlexVector2),
    }
}
/// Possible updates to a single interface element
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InterfaceUpdate {
    #[prost(oneof = "interface_update::Update", tags = "1, 2, 3, 4, 5, 6")]
    pub update: ::core::option::Option<interface_update::Update>,
}
/// Nested message and enum types in `InterfaceUpdate`.
pub mod interface_update {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Update {
        /// Make a copy of this element and set the original to
        /// 'visiblity: hidden'. Subsequent selectors in this sequence will
        /// apply to the cloned element if they search for an element by name.
        #[prost(message, tag = "1")]
        CloneElement(()),
        /// Destroys the element
        #[prost(message, tag = "2")]
        DestroyElement(()),
        /// Animates the element to match the position of another element
        #[prost(message, tag = "3")]
        AnimateToPosition(super::AnimateToPosition),
        /// Immediately apply a style to this element
        #[prost(message, tag = "4")]
        ApplyStyle(super::FlexStyle),
        /// Animates a change to this element's style
        #[prost(message, tag = "5")]
        AnimateStyle(super::AnimateElementStyle),
        /// Creates a 'target' child element to use for position animations
        /// when adding something to a list
        #[prost(message, tag = "6")]
        CreateTargetAtChildIndex(super::CreateTargetAtChildIndex),
    }
}
/// A single, optionally animated, tranformation to an interface element.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateInterfaceStep {
    /// Identifies the element to update
    #[prost(message, optional, tag = "1")]
    pub element: ::core::option::Option<ElementSelector>,
    /// How to mutate the selected element
    #[prost(message, optional, tag = "2")]
    pub update: ::core::option::Option<InterfaceUpdate>,
    /// Delay to introduce before performing this mutation
    #[prost(message, optional, tag = "3")]
    pub start_time: ::core::option::Option<TimeValue>,
}
/// Applies a sequence of user interface element mutations
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateInterfaceCommand {
    #[prost(message, repeated, tag = "1")]
    pub steps: ::prost::alloc::vec::Vec<UpdateInterfaceStep>,
}
/// Boolean-valued expression
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConditionalQuery {
    #[prost(oneof = "conditional_query::Query", tags = "1")]
    pub query: ::core::option::Option<conditional_query::Query>,
}
/// Nested message and enum types in `ConditionalQuery`.
pub mod conditional_query {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Query {
        /// Does a given user interface element exist?
        #[prost(message, tag = "1")]
        ElementExists(super::ElementSelector),
    }
}
/// Conditionally executes one of two command lists based on a boolean query
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConditionalCommand {
    /// Boolean value to evaluate on the client
    #[prost(message, optional, tag = "1")]
    pub query: ::core::option::Option<ConditionalQuery>,
    /// Commands to run if 'query' is true
    #[prost(message, optional, tag = "2")]
    pub if_true: ::core::option::Option<CommandList>,
    /// Commands to run if 'query' is false
    #[prost(message, optional, tag = "3")]
    pub if_false: ::core::option::Option<CommandList>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GameCommand {
    #[prost(
        oneof = "game_command::Command",
        tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 18, 19, 20, 21"
    )]
    pub command: ::core::option::Option<game_command::Command>,
}
/// Nested message and enum types in `GameCommand`.
pub mod game_command {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Command {
        #[prost(message, tag = "1")]
        Debug(super::ClientDebugCommand),
        #[prost(message, tag = "2")]
        Delay(super::DelayCommand),
        #[prost(message, tag = "3")]
        UpdatePanels(super::UpdatePanelsCommand),
        #[prost(message, tag = "4")]
        TogglePanel(super::TogglePanelCommand),
        #[prost(message, tag = "5")]
        UpdateGameView(super::UpdateGameViewCommand),
        #[prost(message, tag = "6")]
        VisitRoom(super::VisitRoomCommand),
        #[prost(message, tag = "7")]
        PlaySound(super::PlaySoundCommand),
        #[prost(message, tag = "8")]
        SetMusic(super::SetMusicCommand),
        #[prost(message, tag = "9")]
        FireProjectile(super::FireProjectileCommand),
        #[prost(message, tag = "10")]
        PlayEffect(super::PlayEffectCommand),
        #[prost(message, tag = "11")]
        DisplayGameMessage(super::DisplayGameMessageCommand),
        #[prost(message, tag = "12")]
        SetGameObjectsEnabled(super::SetGameObjectsEnabledCommand),
        #[prost(message, tag = "13")]
        DisplayRewards(super::DisplayRewardsCommand),
        #[prost(message, tag = "14")]
        LoadScene(super::LoadSceneCommand),
        #[prost(message, tag = "15")]
        MoveGameObjects(super::MoveGameObjectsCommand),
        #[prost(message, tag = "16")]
        CreateTokenCard(super::CreateTokenCardCommand),
        #[prost(message, tag = "18")]
        UpdateWorldMap(super::UpdateWorldMapCommand),
        #[prost(message, tag = "19")]
        RenderScreenOverlay(super::RenderScreenOverlayCommand),
        #[prost(message, tag = "20")]
        UpdateInterface(super::UpdateInterfaceCommand),
        #[prost(message, tag = "21")]
        Conditional(super::ConditionalCommand),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommandList {
    #[prost(message, repeated, tag = "1")]
    pub commands: ::prost::alloc::vec::Vec<GameCommand>,
}
// ============================================================================
// Masonry
// ============================================================================

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FlexAlign {
    Unspecified = 0,
    Auto = 1,
    FlexStart = 2,
    Center = 3,
    FlexEnd = 4,
    Stretch = 5,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FlexDisplayStyle {
    Unspecified = 0,
    Flex = 1,
    None = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FlexDirection {
    Unspecified = 0,
    Column = 1,
    ColumnReverse = 2,
    Row = 3,
    RowReverse = 4,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FlexWrap {
    Unspecified = 0,
    NoWrap = 1,
    Wrap = 2,
    WrapReverse = 3,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FlexJustify {
    Unspecified = 0,
    FlexStart = 1,
    Center = 2,
    FlexEnd = 3,
    SpaceBetween = 4,
    SpaceAround = 5,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FlexOverflow {
    Unspecified = 0,
    Visible = 1,
    Hidden = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FlexPosition {
    Unspecified = 0,
    Relative = 1,
    Absolute = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TextOverflow {
    Unspecified = 0,
    Clip = 1,
    Ellipsis = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EasingMode {
    Unspecified = 0,
    Ease = 1,
    EaseIn = 2,
    EaseOut = 3,
    EaseInOut = 4,
    Linear = 5,
    EaseInSine = 6,
    EaseOutSine = 7,
    EaseInOutSine = 8,
    EaseInCubic = 9,
    EaseOutCubic = 10,
    EaseInOutCubic = 11,
    EaseInCirc = 12,
    EaseOutCirc = 13,
    EaseInOutCirc = 14,
    EaseInElastic = 15,
    EaseOutElastic = 16,
    EaseInOutElastic = 17,
    EaseInBack = 18,
    EaseOutBack = 19,
    EaseInOutBack = 20,
    EaseInBounce = 21,
    EaseOutBounce = 22,
    EaseInOutBounce = 23,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ImageScaleMode {
    Unspecified = 0,
    StretchToFill = 1,
    ScaleAndCrop = 2,
    ScaleToFit = 3,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FontStyle {
    Unspecified = 0,
    Normal = 1,
    Bold = 2,
    Italic = 3,
    BoldAndItalic = 4,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OverflowClipBox {
    Unspecified = 0,
    PaddingBox = 1,
    ContentBox = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TextAlign {
    Unspecified = 0,
    UpperLeft = 1,
    UpperCenter = 2,
    UpperRight = 3,
    MiddleLeft = 4,
    MiddleCenter = 5,
    MiddleRight = 6,
    LowerLeft = 7,
    LowerCenter = 8,
    LowerRight = 9,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TextOverflowPosition {
    Unspecified = 0,
    End = 1,
    Start = 2,
    Middle = 3,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FlexVisibility {
    Unspecified = 0,
    Visible = 1,
    Hidden = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum WhiteSpace {
    Unspecified = 0,
    Normal = 1,
    NoWrap = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DimensionUnit {
    Unspecified = 0,
    /// Measurement in Pixels.
    /// This is Unity density-independent pixels, not real physical pixels.
    Pixels = 1,
    /// Percentage of parent container
    Percentage = 2,
    /// Units relative to 1% of the screen width
    ViewportWidth = 3,
    /// Units relative to 1% of the screen height
    ViewportHeight = 4,
    /// Units relative to 100% of the size of the safe area top inset
    SafeAreaTop = 5,
    /// Units relative to 100% of the size of the safe area right inset
    SafeAreaRight = 6,
    /// Units relative to 100% of the size of the safe area bottom inset
    SafeAreaBottom = 7,
    /// Units relative to 100% of the size of the safe area left inset
    SafeAreaLeft = 8,
}
/// Controls whether elements respond to interface events.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FlexPickingMode {
    /// Unspecified, currently identical to 'position'.
    Unspecified = 0,
    /// Picking enabled, events will be recognized.
    Position = 1,
    /// Picking disabled, events ignored.
    Ignore = 2,
}
/// Allows the size of a Node to be determined by the size of its background
/// sprite.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BackgroundImageAutoSize {
    Unspecified = 0,
    /// Determine the Node height based on its specified width
    FromWidth = 1,
    /// Determine the Node width based on its specified height
    FromHeight = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ScrollBarVisibility {
    Unspecified = 0,
    /// Displays a scroll bar only if the content does not fit in the scroll
    /// view. Otherwise, hides the scroll bar.
    Auto = 1,
    /// The scroll bar is always visible.
    AlwaysVisible = 2,
    /// The scroll bar is always hidden.
    Hidden = 3,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TouchScrollBehavior {
    Unspecified = 0,
    /// The content position can move past the ScrollView boundaries.
    Unrestricted = 1,
    /// The content position can overshoot the ScrollView boundaries, but
    /// then "snaps" back within them.
    Elastic = 2,
    /// The content position is clamped to the ScrollView boundaries.
    Clamped = 3,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SliderDirection {
    Unspecified = 0,
    Horizontal = 1,
    Vertical = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PlayerSide {
    Unspecified = 0,
    Overlord = 1,
    Champion = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PlayerName {
    Unspecified = 0,
    User = 1,
    Opponent = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RoomIdentifier {
    Unspecified = 0,
    Vault = 1,
    Sanctum = 2,
    Crypts = 3,
    RoomA = 4,
    RoomB = 5,
    RoomC = 6,
    RoomD = 7,
    RoomE = 8,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TargetingArrow {
    Unspecified = 0,
    Red = 1,
    Blue = 2,
    Green = 3,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ClientRoomLocation {
    Unspecified = 0,
    Back = 1,
    Front = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ClientItemLocation {
    Unspecified = 0,
    Left = 1,
    Right = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RevealedCardsBrowserSize {
    Unspecified = 0,
    Small = 1,
    Large = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CardPrefab {
    Unspecified = 0,
    Standard = 1,
    TokenCard = 2,
    FullHeight = 3,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ArrowBubbleCorner {
    Unspecified = 0,
    BottomLeft = 1,
    BottomRight = 2,
}
/// Possible corners which can be anchored.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AnchorCorner {
    Unspecified = 0,
    TopLeft = 1,
    TopRight = 2,
    BottomLeft = 3,
    BottomRight = 4,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RoomVisitType {
    Unspecified = 0,
    InitiateRaid = 1,
    LevelUpRoom = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CardCreationAnimation {
    Unspecified = 0,
    /// Animates the card moving from the deck to the staging area.
    DrawCard = 1,
    /// Animates the card moving from its parent card (indicated by its card
    /// identifier with no 'ability_id') to its create position.
    FromParentCard = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MusicState {
    Unspecified = 0,
    Silent = 1,
    Gameplay = 2,
    Raid = 3,
    MainMenu = 4,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum GameMessageType {
    Unspecified = 0,
    Dawn = 1,
    Dusk = 2,
    Victory = 3,
    Defeat = 4,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SceneLoadMode {
    Unspecified = 0,
    /// Close all currently open scenes before loading.
    Single = 1,
    /// Adds a scene to the current loaded scenes.
    Additive = 2,
}
/// Possible client logging levels
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LogMessageLevel {
    Unspecified = 0,
    Standard = 1,
    Warning = 2,
    Error = 3,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MapTileType {
    Unspecified = 0,
    /// Player cannot move through this tile
    Obstacle = 1,
    /// Player can walk through this tile
    Walkable = 2,
    /// Player cannot enter this tile but can click to walk adjacent to it
    Visitable = 3,
}
/// Generated server implementations.
pub mod spelldawn_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for
    /// use with SpelldawnServer.
    #[async_trait]
    pub trait Spelldawn: Send + Sync + 'static {
        ///Server streaming response type for the Connect method.
        type ConnectStream: futures_core::Stream<Item = Result<super::CommandList, tonic::Status>>
            + Send
            + 'static;
        /// Initiate a new server connection.
        async fn connect(
            &self,
            request: tonic::Request<super::ConnectRequest>,
        ) -> Result<tonic::Response<Self::ConnectStream>, tonic::Status>;
        /// Perform a game action.
        async fn perform_action(
            &self,
            request: tonic::Request<super::GameRequest>,
        ) -> Result<tonic::Response<super::CommandList>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct SpelldawnServer<T: Spelldawn> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Spelldawn> SpelldawnServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }

        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }

        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }

        /// Enable decompressing requests with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.accept_compression_encodings.enable_gzip();
            self
        }

        /// Compress responses with `gzip`, if the client supports it.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.send_compression_encodings.enable_gzip();
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for SpelldawnServer<T>
    where
        T: Spelldawn,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        type Response = http::Response<tonic::body::BoxBody>;

        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }

        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/spelldawn.Spelldawn/Connect" => {
                    #[allow(non_camel_case_types)]
                    struct ConnectSvc<T: Spelldawn>(pub Arc<T>);
                    impl<T: Spelldawn> tonic::server::ServerStreamingService<super::ConnectRequest> for ConnectSvc<T> {
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        type Response = super::CommandList;
                        type ResponseStream = T::ConnectStream;

                        fn call(
                            &mut self,
                            request: tonic::Request<super::ConnectRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).connect(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ConnectSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/spelldawn.Spelldawn/PerformAction" => {
                    #[allow(non_camel_case_types)]
                    struct PerformActionSvc<T: Spelldawn>(pub Arc<T>);
                    impl<T: Spelldawn> tonic::server::UnaryService<super::GameRequest> for PerformActionSvc<T> {
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        type Response = super::CommandList;

                        fn call(
                            &mut self,
                            request: tonic::Request<super::GameRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).perform_action(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PerformActionSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Spelldawn> Clone for SpelldawnServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Spelldawn> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Spelldawn> tonic::transport::NamedService for SpelldawnServer<T> {
        const NAME: &'static str = "spelldawn.Spelldawn";
    }
}
