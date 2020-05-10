use amethyst::input::{BindingTypes};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Display, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum AxisBinding {
    Vertical,
    Horizontal,
}

#[derive(Clone, Debug, Display, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionBinding {}

#[derive(Debug)]
pub struct PsychoBindingTypes;

impl BindingTypes for PsychoBindingTypes {
    type Axis = AxisBinding;
    type Action = ActionBinding;
}
