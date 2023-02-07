use std::collections::HashMap;

macro_rules! hashmap {
    ( ) => { HashMap::new() };
    
    ( $kt:ty : $vt:ty ; ) => { HashMap::<$kt, $vt>::new() };

    ( $( $key:tt : $value:tt ),* $(,)? ) => {{
        let mut map = HashMap::new();
        $(
            map.insert($key, $value);
        )*
        
        map
    }};
    
    ( $( & $key:ident : $value:tt ),*  $(,)? ) => {{
        let mut map = HashMap::new();
        $(
            map.insert(&$key, $value);
        )*
        
        map
    }};
    
    ( $( $key:literal : $value:tt ),* $(,)? ) => {{
        let mut map = HashMap::new();
        $(
            map.insert($key, $value);
        )*
        
        map
    }};
    
    ( $kt:ty : $vt:ty ; $( $key:tt : $value:tt ),* $(,)? ) => {{
        let mut map = HashMap::<$kt, $vt>::new();
        $(
            map.insert($key, $value);
        )*
        
        map
    }};
    
    ( $kt:ty : $vt:ty ; $( & $key:ident : $value:tt ),*  $(,)? ) => {{
        let mut map = HashMap::<$kt, $vt>::new();
        $(
            map.insert(&$key, $value);
        )*
        
        map
    }};
    
    ( $kt:ty : $vt:ty ; $( $key:literal : $value:tt ),* $(,)? ) => {{
        let mut map = HashMap::<$kt, $vt>::new();
        $(
            map.insert($key, $value);
        )*
        
        map
    }};
}
