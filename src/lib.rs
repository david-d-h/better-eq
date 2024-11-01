#![feature(macro_metavar_expr)]

#[macro_export]
macro_rules! with_better_eq {($($body:tt)*) => ({
    macro_rules! inner {
        ([$$($$buffer:tt)*] { $$($$inner:tt)* } $$($$tail:tt)*) => (inner!(
            [$$($$buffer)* { with_better_eq!($$($$inner)*) }] $$($$tail)*
        ));
        ([$$($$buffer:tt)*] ( $$($$inner:tt)* ) $$($$tail:tt)*) => (inner!(
            [$$($$buffer)* ( with_better_eq!($$($$inner)*) )] $$($$tail)*
        ));
        ([$$($$buffer:tt)*] [ $$($$inner:tt)* ] $$($$tail:tt)*) => (inner!(
            [$$($$buffer)* [ with_better_eq!($$($$inner)*) ]] $$($$tail)*
        ));
        ([$$($$buffer:tt)*] == $$($$tail:tt)*) => (inner!(
            @eq [$$($$buffer)* ==] $$($$tail)*
        ));
        ([$$($$buffer:tt)*] $$tt:tt $$($$tail:tt)*) => (inner!(
            [$$($$buffer)* $$tt] $$($$tail)*
        ));
        (@eq [$$($$buffer:tt)*] == $$($$tail:tt)*) => (inner!(
            @eq [$$($$buffer)*] $$($$tail)*
        ));
        (@eq [$$($$buffer:tt)*] = $$($$tail:tt)*) => (inner!(
            @eq [$$($$buffer)*] $$($$tail)*
        ));
        (@eq [$$($$buffer:tt)*] $$tt:tt $$($$tail:tt)*) => (inner!(
            [$$($$buffer)* $$tt] $$($$tail)*
        ));
        ([$$($$buffer:tt)*]) => ({ $$($$buffer)* });
    }

    inner!([] $($body)*)
})}

#[test]
#[cfg(test)]
fn it_works() {
    with_better_eq! {
        assert!(1 ========= 1);
    }
}
