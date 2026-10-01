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
    m
}