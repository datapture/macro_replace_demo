#[macro_export]
macro_rules! macro_replace {
    // Estado final del procesamiento
    (@process []{$($tail_block:tt)*},{$($replacement:tt)*}) =>{
        $($tail_block)*
    };

    // replacement
    (@process [@keyword $($head_block:tt)* ]{ $($tail_block:tt)*} , {$($replacement:tt)*}) => {
        macro_replace!(
            @process [$($head_block)* ]{$($tail_block)* $($replacement)* },
            {$($replacement)*}
        )
    };

    // Muncher = consumiendo token por token
    (@process [$token:tt  $($head_block:tt)*  ]{$($tail_block:tt)*} , {$($replacement:tt)*}) => {
        macro_replace!(
            @process [$($head_block)* ]{ $($tail_block)* $token },
            {$($replacement)*}
        )
    };

    ({$($expr_block:tt)*}, {$($replacement:tt)*}) => {
        macro_replace!(@process [$($expr_block)*]{}, {$($replacement)*})
    };
}


#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_works() { 
    // }
}
