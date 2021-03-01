
use amethyst::input::BindingTypes;
use serde::{Serialize, Deserialize};

#[ path = "../../struct/InputBindingTypes.rs" ]
mod state;
pub use state::InputBindingTypes;

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum AxisBinding { }

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionBinding { }

impl BindingTypes for InputBindingTypes {
  type Axis = AxisBinding;
  type Action = ActionBinding;
}


