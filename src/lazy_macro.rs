#[macro_export]
macro_rules! lazy_struct {
    (
        $v:vis struct $lazy_struct:ident {
            uninitialized {
                $($uv:vis $u_field:ident: $u_ty:ty),*
            }

            initialized {
                $($iv:vis $i_field:ident: $i_ty:ty),*
            }

            initializer {
                $uninit:ident => $block:block
            }
        }
    ) => {
        #[derive(Debug)]
        $v struct $lazy_struct {
            $($iv $i_field: $i_ty),*
        }

        paste::paste! {
            pub type [<Lazy $lazy_struct>] = crate::lazy_object::LazyObject<[<$lazy_struct Uninitialized>], $lazy_struct>;

            #[derive(Debug)]
            $v struct [<$lazy_struct Uninitialized>] {
                $($uv $u_field: $u_ty),*
            }

            impl $lazy_struct {
                pub fn new_uninitialized($($u_field: $u_ty),*) -> [<Lazy $lazy_struct>] {
                    let uninitialized = [<$lazy_struct Uninitialized>] {
                        $($u_field),*
                    };

                    crate::lazy_object::LazyObject::new(
                        crate::lazy::Lazy::Uninitialized(uninitialized),
                        Box::new(|$uninit| {
                            $block
                        })
                    )
                }

                pub fn new_uninitialized_generic<$([<$u_field:camel>]: Into<$u_ty>),*>($($u_field: [<$u_field:camel>]),*) -> [<Lazy $lazy_struct>] {
                    let uninitialized = [<$lazy_struct Uninitialized>] {
                        $($u_field: $u_field.into()),*
                    };

                    crate::lazy_object::LazyObject::new(
                        crate::lazy::Lazy::Uninitialized(uninitialized),
                        Box::new(|$uninit| {
                            $block
                        })
                    )
                }
            }
        }
    };
}