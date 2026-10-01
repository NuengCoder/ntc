pub(crate) mod check;
pub(crate) mod dialect;
pub(crate) mod extract;
pub(crate) mod hover_detailed;
pub(crate) mod hover_simple;
pub(crate) mod lexicon;

pub(crate) use self::check::validate;
pub(crate) use self::dialect::{dialect_for_ext, visible_in_dialect, Dialect};
#[allow(unused_imports)]
pub(crate) use self::extract::{
    extract_class_ranges, extract_classes, extract_classes_full, extract_comment_box,
    extract_custom_typed_variables, extract_defines, extract_enums, extract_includes,
    extract_structs, extract_user_functions, extract_variables, in_class_body, ClassInfo,
    DefineInfo, EnumInfo, MemberInfo, UserFunction, VariableInfo,
};
pub(crate) use self::hover_detailed::hover_info_detailed;
pub(crate) use self::hover_simple::hover_info;
pub(crate) use self::lexicon::{builtin_function_names, constant_names, keyword_names, type_names};
