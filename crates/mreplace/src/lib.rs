// Macro privada procesa recursivamente

#[macro_export]
macro_rules! _private_macro_replace {
    // Estado final del procesamiento
    (@process []#{$($tail_block:tt)*}=={$($replacement:tt)*}) => {
        $($tail_block)*
    };

    // replacement
    (@process [@keyword $($head_block:tt)* ]#{ $($tail_block:tt)*}=={$($replacement:tt)*}) => {
        $crate::_private_macro_replace!(
            @process [$($head_block)* ]#{$($tail_block)* $($replacement)* }=={$($replacement)*}
        )
    };    
   

    //- Agrupaciones

    // Agrupacion Parentesis
    (@process [ ( $($group_block:tt)+ ) $($head_block:tt)* ]#{$($tail_block:tt)*}=={$($replacement:tt)*}) => {
        $crate::_private_macro_replace!(
            @process [$($head_block)* ]#{ $($tail_block)* ( $crate::_private_macro_replace!( $($group_block)+ ) ) }
            =={$($replacement)*}
        )
    };

    // // Agrupacion Corchetes cuadrados
    // (@process [ [ $($group_block:tt)+ ] $($head_block:tt)* ]#{$($tail_block:tt)*}=={$($replacement:tt)*}) => {
    //     $crate::_private_macro_replace!(
    //         @process [$($head_block)* ]#{ $($tail_block)* [ $crate::_private_macro_replace!( $($group_block)+ )] }
    //         =={$($replacement)*}
    //     )
    // };

    // // Agrupacion Corchetes curvos
    // (@process [ { $($group_block:tt)+ } $($head_block:tt)* ]#{$($tail_block:tt)*}=={$($replacement:tt)*}) => {
    //     $crate::_private_macro_replace!(
    //         @process [$($head_block)* ]#{ $($tail_block)* { $crate::_private_macro_replace!( $($group_block)+ ) } }
    //         =={$($replacement)*}
    //     )
    // };

    
    //------

    // Muncher = consumiendo token por token
    (@process [ $token:tt $($head_block:tt)* ]#{$($tail_block:tt)*}=={$($replacement:tt)*}) => {
        $crate::_private_macro_replace!(
            @process [$($head_block)* ]#{ $($tail_block)* $token }=={$($replacement)*}
        )
    };


}

#[macro_export]
macro_rules! _macro_replace {    
    // Entry Point ({codigo},{remplazo})
    ({$($expr_block:tt)*},{$($replacement:tt)*}) => {
        $crate::_private_macro_replace!(@process [$($expr_block)*]#{}=={$($replacement)*})
    };
}

pub use _macro_replace as macro_replace;

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_works() { 
    // }
}
