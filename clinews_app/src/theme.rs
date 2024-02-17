use termimad::crossterm::style::{Color::*, Attribute::*};
use termimad::*;


pub fn default() -> MadSkin {


    let mut skin = MadSkin::default();
    skin.bold.set_fg(Yellow);


    skin

}