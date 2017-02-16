//#[macro_export]
macro_rules! hashmap {
  ( $( $a:expr => $b:expr ),* ) => {{
    let mut _map = ::std::collections::HashMap::new();
    $(
      _map.insert($a, $b);
    )*
    _map
  }} 
}

pub mod segments;
pub mod joiners;

#[cfg(test)]
mod tests;
