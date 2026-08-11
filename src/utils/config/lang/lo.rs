use std::collections::HashMap;

pub(crate) mod part01;
pub(crate) mod part02;
pub(crate) mod part03;
pub(crate) mod part04;
pub(crate) mod part05;
pub(crate) mod part06;
pub(crate) mod part07;
pub(crate) mod part08;
pub(crate) mod part09;
pub(crate) mod part10;
pub(crate) mod part11;
pub(crate) mod part12;
pub(crate) mod part13;
pub(crate) mod part14;
pub(crate) mod part15;
pub(crate) mod part16;
pub(crate) mod part17;
pub(crate) mod part18;

pub fn build_map() -> HashMap<String, String> {
    let mut m = HashMap::new();
    part01::fill(&mut m);
    part02::fill(&mut m);
    part03::fill(&mut m);
    part04::fill(&mut m);
    part05::fill(&mut m);
    part06::fill(&mut m);
    part07::fill(&mut m);
    part08::fill(&mut m);
    part09::fill(&mut m);
    part10::fill(&mut m);
    part11::fill(&mut m);
    part12::fill(&mut m);
    part13::fill(&mut m);
    part14::fill(&mut m);
    part15::fill(&mut m);
    part16::fill(&mut m);
    part17::fill(&mut m);
    part18::fill(&mut m);
    m
}