#[macro_export]
macro_rules! unwrap {
    ($name:ident : $tpe:ty) => {
        fn $name(&mut self) -> &mut godot::obj::Gd<$tpe> {
            match &mut self.$name {
                Some($name) => $name,
                None => panic!("missing {}", stringify!($name)),
            }
        }
    };
}
