use users::Group;
use users::User;

use crate::mode::ModeT;


#[derive(Debug)]
pub struct Changes<'o> {
    pub owner: Option<&'o User>,
    pub group: Option<&'o Group>,
    pub mode: Option<ModeT>,
}
