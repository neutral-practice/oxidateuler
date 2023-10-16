pub mod display_mods {
	
	pub trait Groupable {
	    fn group_with_nothing(&self) -> String;
	}
	
	impl Groupable for u128 {
	    fn group_with_nothing(&self) -> String {
	        self
	        .to_string()                             // lol
	        .as_bytes()                              // this is 
	        .rchunks(3)                              // how
	        .rev()                                   // we 
	        .map(std::str::from_utf8)                // format large numbers
	        .collect::<Result<Vec<&str>, _>>()       // to visually readable formats
	        .unwrap()                                // in rust
	        .join(" ")                               // and nobody minds this
	    }
	}

}