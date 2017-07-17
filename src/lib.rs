#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_cbor;
pub trait Named { fn name(&self) -> &str; }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E0 { A, B, C }
impl Default for E0 { fn default() -> E0 { E0::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F0 { A, B, C }
impl Default for F0 { fn default() -> F0 { F0::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub0 { pub e: E0, pub f: F0, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub0 { pub fn new(name: String) -> Sub0 { Sub0{e: Default::default(), f: F0::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type0 { pub sub: Sub0, pub e: E0, pub f: F0, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type0 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type0{sub: Default::default(), e: Default::default(), f: F0::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type0 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E1 { A, B, C }
impl Default for E1 { fn default() -> E1 { E1::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F1 { A, B, C }
impl Default for F1 { fn default() -> F1 { F1::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub1 { pub e: E1, pub f: F1, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub1 { pub fn new(name: String) -> Sub1 { Sub1{e: Default::default(), f: F1::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type1 { pub sub: Sub1, pub e: E1, pub f: F1, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type1 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type1{sub: Default::default(), e: Default::default(), f: F1::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type1 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E2 { A, B, C }
impl Default for E2 { fn default() -> E2 { E2::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F2 { A, B, C }
impl Default for F2 { fn default() -> F2 { F2::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub2 { pub e: E2, pub f: F2, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub2 { pub fn new(name: String) -> Sub2 { Sub2{e: Default::default(), f: F2::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type2 { pub sub: Sub2, pub e: E2, pub f: F2, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type2 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type2{sub: Default::default(), e: Default::default(), f: F2::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type2 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E3 { A, B, C }
impl Default for E3 { fn default() -> E3 { E3::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F3 { A, B, C }
impl Default for F3 { fn default() -> F3 { F3::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub3 { pub e: E3, pub f: F3, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub3 { pub fn new(name: String) -> Sub3 { Sub3{e: Default::default(), f: F3::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type3 { pub sub: Sub3, pub e: E3, pub f: F3, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type3 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type3{sub: Default::default(), e: Default::default(), f: F3::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type3 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E4 { A, B, C }
impl Default for E4 { fn default() -> E4 { E4::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F4 { A, B, C }
impl Default for F4 { fn default() -> F4 { F4::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub4 { pub e: E4, pub f: F4, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub4 { pub fn new(name: String) -> Sub4 { Sub4{e: Default::default(), f: F4::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type4 { pub sub: Sub4, pub e: E4, pub f: F4, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type4 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type4{sub: Default::default(), e: Default::default(), f: F4::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type4 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E5 { A, B, C }
impl Default for E5 { fn default() -> E5 { E5::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F5 { A, B, C }
impl Default for F5 { fn default() -> F5 { F5::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub5 { pub e: E5, pub f: F5, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub5 { pub fn new(name: String) -> Sub5 { Sub5{e: Default::default(), f: F5::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type5 { pub sub: Sub5, pub e: E5, pub f: F5, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type5 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type5{sub: Default::default(), e: Default::default(), f: F5::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type5 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E6 { A, B, C }
impl Default for E6 { fn default() -> E6 { E6::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F6 { A, B, C }
impl Default for F6 { fn default() -> F6 { F6::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub6 { pub e: E6, pub f: F6, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub6 { pub fn new(name: String) -> Sub6 { Sub6{e: Default::default(), f: F6::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type6 { pub sub: Sub6, pub e: E6, pub f: F6, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type6 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type6{sub: Default::default(), e: Default::default(), f: F6::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type6 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E7 { A, B, C }
impl Default for E7 { fn default() -> E7 { E7::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F7 { A, B, C }
impl Default for F7 { fn default() -> F7 { F7::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub7 { pub e: E7, pub f: F7, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub7 { pub fn new(name: String) -> Sub7 { Sub7{e: Default::default(), f: F7::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type7 { pub sub: Sub7, pub e: E7, pub f: F7, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type7 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type7{sub: Default::default(), e: Default::default(), f: F7::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type7 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E8 { A, B, C }
impl Default for E8 { fn default() -> E8 { E8::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F8 { A, B, C }
impl Default for F8 { fn default() -> F8 { F8::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub8 { pub e: E8, pub f: F8, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub8 { pub fn new(name: String) -> Sub8 { Sub8{e: Default::default(), f: F8::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type8 { pub sub: Sub8, pub e: E8, pub f: F8, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type8 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type8{sub: Default::default(), e: Default::default(), f: F8::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type8 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E9 { A, B, C }
impl Default for E9 { fn default() -> E9 { E9::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F9 { A, B, C }
impl Default for F9 { fn default() -> F9 { F9::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub9 { pub e: E9, pub f: F9, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub9 { pub fn new(name: String) -> Sub9 { Sub9{e: Default::default(), f: F9::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type9 { pub sub: Sub9, pub e: E9, pub f: F9, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type9 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type9{sub: Default::default(), e: Default::default(), f: F9::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type9 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E10 { A, B, C }
impl Default for E10 { fn default() -> E10 { E10::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F10 { A, B, C }
impl Default for F10 { fn default() -> F10 { F10::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub10 { pub e: E10, pub f: F10, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub10 { pub fn new(name: String) -> Sub10 { Sub10{e: Default::default(), f: F10::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type10 { pub sub: Sub10, pub e: E10, pub f: F10, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type10 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type10{sub: Default::default(), e: Default::default(), f: F10::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type10 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E11 { A, B, C }
impl Default for E11 { fn default() -> E11 { E11::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F11 { A, B, C }
impl Default for F11 { fn default() -> F11 { F11::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub11 { pub e: E11, pub f: F11, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub11 { pub fn new(name: String) -> Sub11 { Sub11{e: Default::default(), f: F11::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type11 { pub sub: Sub11, pub e: E11, pub f: F11, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type11 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type11{sub: Default::default(), e: Default::default(), f: F11::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type11 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E12 { A, B, C }
impl Default for E12 { fn default() -> E12 { E12::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F12 { A, B, C }
impl Default for F12 { fn default() -> F12 { F12::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub12 { pub e: E12, pub f: F12, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub12 { pub fn new(name: String) -> Sub12 { Sub12{e: Default::default(), f: F12::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type12 { pub sub: Sub12, pub e: E12, pub f: F12, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type12 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type12{sub: Default::default(), e: Default::default(), f: F12::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type12 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E13 { A, B, C }
impl Default for E13 { fn default() -> E13 { E13::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F13 { A, B, C }
impl Default for F13 { fn default() -> F13 { F13::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub13 { pub e: E13, pub f: F13, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub13 { pub fn new(name: String) -> Sub13 { Sub13{e: Default::default(), f: F13::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type13 { pub sub: Sub13, pub e: E13, pub f: F13, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type13 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type13{sub: Default::default(), e: Default::default(), f: F13::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type13 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E14 { A, B, C }
impl Default for E14 { fn default() -> E14 { E14::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F14 { A, B, C }
impl Default for F14 { fn default() -> F14 { F14::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub14 { pub e: E14, pub f: F14, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub14 { pub fn new(name: String) -> Sub14 { Sub14{e: Default::default(), f: F14::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type14 { pub sub: Sub14, pub e: E14, pub f: F14, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type14 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type14{sub: Default::default(), e: Default::default(), f: F14::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type14 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E15 { A, B, C }
impl Default for E15 { fn default() -> E15 { E15::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F15 { A, B, C }
impl Default for F15 { fn default() -> F15 { F15::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub15 { pub e: E15, pub f: F15, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub15 { pub fn new(name: String) -> Sub15 { Sub15{e: Default::default(), f: F15::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type15 { pub sub: Sub15, pub e: E15, pub f: F15, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type15 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type15{sub: Default::default(), e: Default::default(), f: F15::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type15 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E16 { A, B, C }
impl Default for E16 { fn default() -> E16 { E16::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F16 { A, B, C }
impl Default for F16 { fn default() -> F16 { F16::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub16 { pub e: E16, pub f: F16, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub16 { pub fn new(name: String) -> Sub16 { Sub16{e: Default::default(), f: F16::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type16 { pub sub: Sub16, pub e: E16, pub f: F16, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type16 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type16{sub: Default::default(), e: Default::default(), f: F16::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type16 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E17 { A, B, C }
impl Default for E17 { fn default() -> E17 { E17::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F17 { A, B, C }
impl Default for F17 { fn default() -> F17 { F17::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub17 { pub e: E17, pub f: F17, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub17 { pub fn new(name: String) -> Sub17 { Sub17{e: Default::default(), f: F17::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type17 { pub sub: Sub17, pub e: E17, pub f: F17, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type17 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type17{sub: Default::default(), e: Default::default(), f: F17::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type17 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E18 { A, B, C }
impl Default for E18 { fn default() -> E18 { E18::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F18 { A, B, C }
impl Default for F18 { fn default() -> F18 { F18::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub18 { pub e: E18, pub f: F18, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub18 { pub fn new(name: String) -> Sub18 { Sub18{e: Default::default(), f: F18::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type18 { pub sub: Sub18, pub e: E18, pub f: F18, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type18 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type18{sub: Default::default(), e: Default::default(), f: F18::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type18 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E19 { A, B, C }
impl Default for E19 { fn default() -> E19 { E19::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F19 { A, B, C }
impl Default for F19 { fn default() -> F19 { F19::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub19 { pub e: E19, pub f: F19, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub19 { pub fn new(name: String) -> Sub19 { Sub19{e: Default::default(), f: F19::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type19 { pub sub: Sub19, pub e: E19, pub f: F19, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type19 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type19{sub: Default::default(), e: Default::default(), f: F19::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type19 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E20 { A, B, C }
impl Default for E20 { fn default() -> E20 { E20::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F20 { A, B, C }
impl Default for F20 { fn default() -> F20 { F20::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub20 { pub e: E20, pub f: F20, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub20 { pub fn new(name: String) -> Sub20 { Sub20{e: Default::default(), f: F20::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type20 { pub sub: Sub20, pub e: E20, pub f: F20, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type20 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type20{sub: Default::default(), e: Default::default(), f: F20::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type20 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E21 { A, B, C }
impl Default for E21 { fn default() -> E21 { E21::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F21 { A, B, C }
impl Default for F21 { fn default() -> F21 { F21::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub21 { pub e: E21, pub f: F21, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub21 { pub fn new(name: String) -> Sub21 { Sub21{e: Default::default(), f: F21::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type21 { pub sub: Sub21, pub e: E21, pub f: F21, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type21 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type21{sub: Default::default(), e: Default::default(), f: F21::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type21 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E22 { A, B, C }
impl Default for E22 { fn default() -> E22 { E22::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F22 { A, B, C }
impl Default for F22 { fn default() -> F22 { F22::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub22 { pub e: E22, pub f: F22, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub22 { pub fn new(name: String) -> Sub22 { Sub22{e: Default::default(), f: F22::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type22 { pub sub: Sub22, pub e: E22, pub f: F22, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type22 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type22{sub: Default::default(), e: Default::default(), f: F22::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type22 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E23 { A, B, C }
impl Default for E23 { fn default() -> E23 { E23::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F23 { A, B, C }
impl Default for F23 { fn default() -> F23 { F23::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub23 { pub e: E23, pub f: F23, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub23 { pub fn new(name: String) -> Sub23 { Sub23{e: Default::default(), f: F23::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type23 { pub sub: Sub23, pub e: E23, pub f: F23, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type23 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type23{sub: Default::default(), e: Default::default(), f: F23::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type23 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E24 { A, B, C }
impl Default for E24 { fn default() -> E24 { E24::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F24 { A, B, C }
impl Default for F24 { fn default() -> F24 { F24::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub24 { pub e: E24, pub f: F24, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub24 { pub fn new(name: String) -> Sub24 { Sub24{e: Default::default(), f: F24::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type24 { pub sub: Sub24, pub e: E24, pub f: F24, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type24 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type24{sub: Default::default(), e: Default::default(), f: F24::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type24 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E25 { A, B, C }
impl Default for E25 { fn default() -> E25 { E25::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F25 { A, B, C }
impl Default for F25 { fn default() -> F25 { F25::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub25 { pub e: E25, pub f: F25, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub25 { pub fn new(name: String) -> Sub25 { Sub25{e: Default::default(), f: F25::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type25 { pub sub: Sub25, pub e: E25, pub f: F25, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type25 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type25{sub: Default::default(), e: Default::default(), f: F25::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type25 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E26 { A, B, C }
impl Default for E26 { fn default() -> E26 { E26::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F26 { A, B, C }
impl Default for F26 { fn default() -> F26 { F26::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub26 { pub e: E26, pub f: F26, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub26 { pub fn new(name: String) -> Sub26 { Sub26{e: Default::default(), f: F26::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type26 { pub sub: Sub26, pub e: E26, pub f: F26, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type26 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type26{sub: Default::default(), e: Default::default(), f: F26::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type26 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E27 { A, B, C }
impl Default for E27 { fn default() -> E27 { E27::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F27 { A, B, C }
impl Default for F27 { fn default() -> F27 { F27::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub27 { pub e: E27, pub f: F27, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub27 { pub fn new(name: String) -> Sub27 { Sub27{e: Default::default(), f: F27::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type27 { pub sub: Sub27, pub e: E27, pub f: F27, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type27 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type27{sub: Default::default(), e: Default::default(), f: F27::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type27 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E28 { A, B, C }
impl Default for E28 { fn default() -> E28 { E28::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F28 { A, B, C }
impl Default for F28 { fn default() -> F28 { F28::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub28 { pub e: E28, pub f: F28, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub28 { pub fn new(name: String) -> Sub28 { Sub28{e: Default::default(), f: F28::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type28 { pub sub: Sub28, pub e: E28, pub f: F28, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type28 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type28{sub: Default::default(), e: Default::default(), f: F28::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type28 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E29 { A, B, C }
impl Default for E29 { fn default() -> E29 { E29::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F29 { A, B, C }
impl Default for F29 { fn default() -> F29 { F29::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub29 { pub e: E29, pub f: F29, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub29 { pub fn new(name: String) -> Sub29 { Sub29{e: Default::default(), f: F29::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type29 { pub sub: Sub29, pub e: E29, pub f: F29, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type29 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type29{sub: Default::default(), e: Default::default(), f: F29::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type29 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E30 { A, B, C }
impl Default for E30 { fn default() -> E30 { E30::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F30 { A, B, C }
impl Default for F30 { fn default() -> F30 { F30::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub30 { pub e: E30, pub f: F30, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub30 { pub fn new(name: String) -> Sub30 { Sub30{e: Default::default(), f: F30::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type30 { pub sub: Sub30, pub e: E30, pub f: F30, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type30 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type30{sub: Default::default(), e: Default::default(), f: F30::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type30 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E31 { A, B, C }
impl Default for E31 { fn default() -> E31 { E31::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F31 { A, B, C }
impl Default for F31 { fn default() -> F31 { F31::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub31 { pub e: E31, pub f: F31, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub31 { pub fn new(name: String) -> Sub31 { Sub31{e: Default::default(), f: F31::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type31 { pub sub: Sub31, pub e: E31, pub f: F31, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type31 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type31{sub: Default::default(), e: Default::default(), f: F31::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type31 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E32 { A, B, C }
impl Default for E32 { fn default() -> E32 { E32::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F32 { A, B, C }
impl Default for F32 { fn default() -> F32 { F32::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub32 { pub e: E32, pub f: F32, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub32 { pub fn new(name: String) -> Sub32 { Sub32{e: Default::default(), f: F32::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type32 { pub sub: Sub32, pub e: E32, pub f: F32, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type32 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type32{sub: Default::default(), e: Default::default(), f: F32::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type32 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E33 { A, B, C }
impl Default for E33 { fn default() -> E33 { E33::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F33 { A, B, C }
impl Default for F33 { fn default() -> F33 { F33::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub33 { pub e: E33, pub f: F33, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub33 { pub fn new(name: String) -> Sub33 { Sub33{e: Default::default(), f: F33::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type33 { pub sub: Sub33, pub e: E33, pub f: F33, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type33 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type33{sub: Default::default(), e: Default::default(), f: F33::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type33 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E34 { A, B, C }
impl Default for E34 { fn default() -> E34 { E34::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F34 { A, B, C }
impl Default for F34 { fn default() -> F34 { F34::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub34 { pub e: E34, pub f: F34, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub34 { pub fn new(name: String) -> Sub34 { Sub34{e: Default::default(), f: F34::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type34 { pub sub: Sub34, pub e: E34, pub f: F34, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type34 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type34{sub: Default::default(), e: Default::default(), f: F34::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type34 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E35 { A, B, C }
impl Default for E35 { fn default() -> E35 { E35::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F35 { A, B, C }
impl Default for F35 { fn default() -> F35 { F35::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub35 { pub e: E35, pub f: F35, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub35 { pub fn new(name: String) -> Sub35 { Sub35{e: Default::default(), f: F35::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type35 { pub sub: Sub35, pub e: E35, pub f: F35, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type35 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type35{sub: Default::default(), e: Default::default(), f: F35::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type35 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E36 { A, B, C }
impl Default for E36 { fn default() -> E36 { E36::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F36 { A, B, C }
impl Default for F36 { fn default() -> F36 { F36::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub36 { pub e: E36, pub f: F36, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub36 { pub fn new(name: String) -> Sub36 { Sub36{e: Default::default(), f: F36::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type36 { pub sub: Sub36, pub e: E36, pub f: F36, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type36 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type36{sub: Default::default(), e: Default::default(), f: F36::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type36 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E37 { A, B, C }
impl Default for E37 { fn default() -> E37 { E37::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F37 { A, B, C }
impl Default for F37 { fn default() -> F37 { F37::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub37 { pub e: E37, pub f: F37, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub37 { pub fn new(name: String) -> Sub37 { Sub37{e: Default::default(), f: F37::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type37 { pub sub: Sub37, pub e: E37, pub f: F37, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type37 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type37{sub: Default::default(), e: Default::default(), f: F37::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type37 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E38 { A, B, C }
impl Default for E38 { fn default() -> E38 { E38::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F38 { A, B, C }
impl Default for F38 { fn default() -> F38 { F38::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub38 { pub e: E38, pub f: F38, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub38 { pub fn new(name: String) -> Sub38 { Sub38{e: Default::default(), f: F38::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type38 { pub sub: Sub38, pub e: E38, pub f: F38, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type38 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type38{sub: Default::default(), e: Default::default(), f: F38::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type38 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E39 { A, B, C }
impl Default for E39 { fn default() -> E39 { E39::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F39 { A, B, C }
impl Default for F39 { fn default() -> F39 { F39::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub39 { pub e: E39, pub f: F39, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub39 { pub fn new(name: String) -> Sub39 { Sub39{e: Default::default(), f: F39::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type39 { pub sub: Sub39, pub e: E39, pub f: F39, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type39 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type39{sub: Default::default(), e: Default::default(), f: F39::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type39 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E40 { A, B, C }
impl Default for E40 { fn default() -> E40 { E40::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F40 { A, B, C }
impl Default for F40 { fn default() -> F40 { F40::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub40 { pub e: E40, pub f: F40, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub40 { pub fn new(name: String) -> Sub40 { Sub40{e: Default::default(), f: F40::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type40 { pub sub: Sub40, pub e: E40, pub f: F40, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type40 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type40{sub: Default::default(), e: Default::default(), f: F40::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type40 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E41 { A, B, C }
impl Default for E41 { fn default() -> E41 { E41::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F41 { A, B, C }
impl Default for F41 { fn default() -> F41 { F41::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub41 { pub e: E41, pub f: F41, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub41 { pub fn new(name: String) -> Sub41 { Sub41{e: Default::default(), f: F41::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type41 { pub sub: Sub41, pub e: E41, pub f: F41, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type41 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type41{sub: Default::default(), e: Default::default(), f: F41::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type41 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E42 { A, B, C }
impl Default for E42 { fn default() -> E42 { E42::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F42 { A, B, C }
impl Default for F42 { fn default() -> F42 { F42::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub42 { pub e: E42, pub f: F42, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub42 { pub fn new(name: String) -> Sub42 { Sub42{e: Default::default(), f: F42::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type42 { pub sub: Sub42, pub e: E42, pub f: F42, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type42 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type42{sub: Default::default(), e: Default::default(), f: F42::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type42 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E43 { A, B, C }
impl Default for E43 { fn default() -> E43 { E43::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F43 { A, B, C }
impl Default for F43 { fn default() -> F43 { F43::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub43 { pub e: E43, pub f: F43, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub43 { pub fn new(name: String) -> Sub43 { Sub43{e: Default::default(), f: F43::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type43 { pub sub: Sub43, pub e: E43, pub f: F43, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type43 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type43{sub: Default::default(), e: Default::default(), f: F43::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type43 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E44 { A, B, C }
impl Default for E44 { fn default() -> E44 { E44::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F44 { A, B, C }
impl Default for F44 { fn default() -> F44 { F44::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub44 { pub e: E44, pub f: F44, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub44 { pub fn new(name: String) -> Sub44 { Sub44{e: Default::default(), f: F44::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type44 { pub sub: Sub44, pub e: E44, pub f: F44, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type44 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type44{sub: Default::default(), e: Default::default(), f: F44::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type44 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E45 { A, B, C }
impl Default for E45 { fn default() -> E45 { E45::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F45 { A, B, C }
impl Default for F45 { fn default() -> F45 { F45::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub45 { pub e: E45, pub f: F45, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub45 { pub fn new(name: String) -> Sub45 { Sub45{e: Default::default(), f: F45::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type45 { pub sub: Sub45, pub e: E45, pub f: F45, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type45 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type45{sub: Default::default(), e: Default::default(), f: F45::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type45 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E46 { A, B, C }
impl Default for E46 { fn default() -> E46 { E46::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F46 { A, B, C }
impl Default for F46 { fn default() -> F46 { F46::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub46 { pub e: E46, pub f: F46, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub46 { pub fn new(name: String) -> Sub46 { Sub46{e: Default::default(), f: F46::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type46 { pub sub: Sub46, pub e: E46, pub f: F46, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type46 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type46{sub: Default::default(), e: Default::default(), f: F46::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type46 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E47 { A, B, C }
impl Default for E47 { fn default() -> E47 { E47::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F47 { A, B, C }
impl Default for F47 { fn default() -> F47 { F47::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub47 { pub e: E47, pub f: F47, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub47 { pub fn new(name: String) -> Sub47 { Sub47{e: Default::default(), f: F47::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type47 { pub sub: Sub47, pub e: E47, pub f: F47, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type47 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type47{sub: Default::default(), e: Default::default(), f: F47::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type47 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E48 { A, B, C }
impl Default for E48 { fn default() -> E48 { E48::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F48 { A, B, C }
impl Default for F48 { fn default() -> F48 { F48::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub48 { pub e: E48, pub f: F48, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub48 { pub fn new(name: String) -> Sub48 { Sub48{e: Default::default(), f: F48::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type48 { pub sub: Sub48, pub e: E48, pub f: F48, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type48 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type48{sub: Default::default(), e: Default::default(), f: F48::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type48 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E49 { A, B, C }
impl Default for E49 { fn default() -> E49 { E49::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F49 { A, B, C }
impl Default for F49 { fn default() -> F49 { F49::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub49 { pub e: E49, pub f: F49, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub49 { pub fn new(name: String) -> Sub49 { Sub49{e: Default::default(), f: F49::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type49 { pub sub: Sub49, pub e: E49, pub f: F49, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type49 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type49{sub: Default::default(), e: Default::default(), f: F49::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type49 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E50 { A, B, C }
impl Default for E50 { fn default() -> E50 { E50::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F50 { A, B, C }
impl Default for F50 { fn default() -> F50 { F50::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub50 { pub e: E50, pub f: F50, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub50 { pub fn new(name: String) -> Sub50 { Sub50{e: Default::default(), f: F50::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type50 { pub sub: Sub50, pub e: E50, pub f: F50, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type50 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type50{sub: Default::default(), e: Default::default(), f: F50::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type50 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E51 { A, B, C }
impl Default for E51 { fn default() -> E51 { E51::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F51 { A, B, C }
impl Default for F51 { fn default() -> F51 { F51::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub51 { pub e: E51, pub f: F51, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub51 { pub fn new(name: String) -> Sub51 { Sub51{e: Default::default(), f: F51::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type51 { pub sub: Sub51, pub e: E51, pub f: F51, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type51 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type51{sub: Default::default(), e: Default::default(), f: F51::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type51 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E52 { A, B, C }
impl Default for E52 { fn default() -> E52 { E52::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F52 { A, B, C }
impl Default for F52 { fn default() -> F52 { F52::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub52 { pub e: E52, pub f: F52, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub52 { pub fn new(name: String) -> Sub52 { Sub52{e: Default::default(), f: F52::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type52 { pub sub: Sub52, pub e: E52, pub f: F52, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type52 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type52{sub: Default::default(), e: Default::default(), f: F52::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type52 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E53 { A, B, C }
impl Default for E53 { fn default() -> E53 { E53::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F53 { A, B, C }
impl Default for F53 { fn default() -> F53 { F53::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub53 { pub e: E53, pub f: F53, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub53 { pub fn new(name: String) -> Sub53 { Sub53{e: Default::default(), f: F53::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type53 { pub sub: Sub53, pub e: E53, pub f: F53, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type53 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type53{sub: Default::default(), e: Default::default(), f: F53::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type53 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E54 { A, B, C }
impl Default for E54 { fn default() -> E54 { E54::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F54 { A, B, C }
impl Default for F54 { fn default() -> F54 { F54::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub54 { pub e: E54, pub f: F54, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub54 { pub fn new(name: String) -> Sub54 { Sub54{e: Default::default(), f: F54::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type54 { pub sub: Sub54, pub e: E54, pub f: F54, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type54 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type54{sub: Default::default(), e: Default::default(), f: F54::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type54 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E55 { A, B, C }
impl Default for E55 { fn default() -> E55 { E55::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F55 { A, B, C }
impl Default for F55 { fn default() -> F55 { F55::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub55 { pub e: E55, pub f: F55, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub55 { pub fn new(name: String) -> Sub55 { Sub55{e: Default::default(), f: F55::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type55 { pub sub: Sub55, pub e: E55, pub f: F55, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type55 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type55{sub: Default::default(), e: Default::default(), f: F55::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type55 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E56 { A, B, C }
impl Default for E56 { fn default() -> E56 { E56::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F56 { A, B, C }
impl Default for F56 { fn default() -> F56 { F56::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub56 { pub e: E56, pub f: F56, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub56 { pub fn new(name: String) -> Sub56 { Sub56{e: Default::default(), f: F56::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type56 { pub sub: Sub56, pub e: E56, pub f: F56, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type56 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type56{sub: Default::default(), e: Default::default(), f: F56::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type56 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E57 { A, B, C }
impl Default for E57 { fn default() -> E57 { E57::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F57 { A, B, C }
impl Default for F57 { fn default() -> F57 { F57::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub57 { pub e: E57, pub f: F57, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub57 { pub fn new(name: String) -> Sub57 { Sub57{e: Default::default(), f: F57::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type57 { pub sub: Sub57, pub e: E57, pub f: F57, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type57 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type57{sub: Default::default(), e: Default::default(), f: F57::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type57 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E58 { A, B, C }
impl Default for E58 { fn default() -> E58 { E58::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F58 { A, B, C }
impl Default for F58 { fn default() -> F58 { F58::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub58 { pub e: E58, pub f: F58, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub58 { pub fn new(name: String) -> Sub58 { Sub58{e: Default::default(), f: F58::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type58 { pub sub: Sub58, pub e: E58, pub f: F58, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type58 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type58{sub: Default::default(), e: Default::default(), f: F58::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type58 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E59 { A, B, C }
impl Default for E59 { fn default() -> E59 { E59::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F59 { A, B, C }
impl Default for F59 { fn default() -> F59 { F59::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub59 { pub e: E59, pub f: F59, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub59 { pub fn new(name: String) -> Sub59 { Sub59{e: Default::default(), f: F59::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type59 { pub sub: Sub59, pub e: E59, pub f: F59, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type59 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type59{sub: Default::default(), e: Default::default(), f: F59::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type59 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E60 { A, B, C }
impl Default for E60 { fn default() -> E60 { E60::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F60 { A, B, C }
impl Default for F60 { fn default() -> F60 { F60::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub60 { pub e: E60, pub f: F60, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub60 { pub fn new(name: String) -> Sub60 { Sub60{e: Default::default(), f: F60::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type60 { pub sub: Sub60, pub e: E60, pub f: F60, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type60 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type60{sub: Default::default(), e: Default::default(), f: F60::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type60 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E61 { A, B, C }
impl Default for E61 { fn default() -> E61 { E61::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F61 { A, B, C }
impl Default for F61 { fn default() -> F61 { F61::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub61 { pub e: E61, pub f: F61, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub61 { pub fn new(name: String) -> Sub61 { Sub61{e: Default::default(), f: F61::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type61 { pub sub: Sub61, pub e: E61, pub f: F61, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type61 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type61{sub: Default::default(), e: Default::default(), f: F61::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type61 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E62 { A, B, C }
impl Default for E62 { fn default() -> E62 { E62::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F62 { A, B, C }
impl Default for F62 { fn default() -> F62 { F62::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub62 { pub e: E62, pub f: F62, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub62 { pub fn new(name: String) -> Sub62 { Sub62{e: Default::default(), f: F62::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type62 { pub sub: Sub62, pub e: E62, pub f: F62, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type62 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type62{sub: Default::default(), e: Default::default(), f: F62::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type62 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E63 { A, B, C }
impl Default for E63 { fn default() -> E63 { E63::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F63 { A, B, C }
impl Default for F63 { fn default() -> F63 { F63::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub63 { pub e: E63, pub f: F63, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub63 { pub fn new(name: String) -> Sub63 { Sub63{e: Default::default(), f: F63::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type63 { pub sub: Sub63, pub e: E63, pub f: F63, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type63 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type63{sub: Default::default(), e: Default::default(), f: F63::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type63 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E64 { A, B, C }
impl Default for E64 { fn default() -> E64 { E64::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F64 { A, B, C }
impl Default for F64 { fn default() -> F64 { F64::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub64 { pub e: E64, pub f: F64, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub64 { pub fn new(name: String) -> Sub64 { Sub64{e: Default::default(), f: F64::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type64 { pub sub: Sub64, pub e: E64, pub f: F64, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type64 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type64{sub: Default::default(), e: Default::default(), f: F64::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type64 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E65 { A, B, C }
impl Default for E65 { fn default() -> E65 { E65::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F65 { A, B, C }
impl Default for F65 { fn default() -> F65 { F65::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub65 { pub e: E65, pub f: F65, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub65 { pub fn new(name: String) -> Sub65 { Sub65{e: Default::default(), f: F65::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type65 { pub sub: Sub65, pub e: E65, pub f: F65, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type65 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type65{sub: Default::default(), e: Default::default(), f: F65::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type65 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E66 { A, B, C }
impl Default for E66 { fn default() -> E66 { E66::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F66 { A, B, C }
impl Default for F66 { fn default() -> F66 { F66::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub66 { pub e: E66, pub f: F66, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub66 { pub fn new(name: String) -> Sub66 { Sub66{e: Default::default(), f: F66::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type66 { pub sub: Sub66, pub e: E66, pub f: F66, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type66 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type66{sub: Default::default(), e: Default::default(), f: F66::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type66 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E67 { A, B, C }
impl Default for E67 { fn default() -> E67 { E67::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F67 { A, B, C }
impl Default for F67 { fn default() -> F67 { F67::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub67 { pub e: E67, pub f: F67, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub67 { pub fn new(name: String) -> Sub67 { Sub67{e: Default::default(), f: F67::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type67 { pub sub: Sub67, pub e: E67, pub f: F67, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type67 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type67{sub: Default::default(), e: Default::default(), f: F67::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type67 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E68 { A, B, C }
impl Default for E68 { fn default() -> E68 { E68::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F68 { A, B, C }
impl Default for F68 { fn default() -> F68 { F68::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub68 { pub e: E68, pub f: F68, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub68 { pub fn new(name: String) -> Sub68 { Sub68{e: Default::default(), f: F68::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type68 { pub sub: Sub68, pub e: E68, pub f: F68, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type68 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type68{sub: Default::default(), e: Default::default(), f: F68::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type68 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E69 { A, B, C }
impl Default for E69 { fn default() -> E69 { E69::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F69 { A, B, C }
impl Default for F69 { fn default() -> F69 { F69::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub69 { pub e: E69, pub f: F69, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub69 { pub fn new(name: String) -> Sub69 { Sub69{e: Default::default(), f: F69::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type69 { pub sub: Sub69, pub e: E69, pub f: F69, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type69 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type69{sub: Default::default(), e: Default::default(), f: F69::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type69 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E70 { A, B, C }
impl Default for E70 { fn default() -> E70 { E70::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F70 { A, B, C }
impl Default for F70 { fn default() -> F70 { F70::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub70 { pub e: E70, pub f: F70, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub70 { pub fn new(name: String) -> Sub70 { Sub70{e: Default::default(), f: F70::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type70 { pub sub: Sub70, pub e: E70, pub f: F70, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type70 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type70{sub: Default::default(), e: Default::default(), f: F70::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type70 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E71 { A, B, C }
impl Default for E71 { fn default() -> E71 { E71::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F71 { A, B, C }
impl Default for F71 { fn default() -> F71 { F71::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub71 { pub e: E71, pub f: F71, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub71 { pub fn new(name: String) -> Sub71 { Sub71{e: Default::default(), f: F71::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type71 { pub sub: Sub71, pub e: E71, pub f: F71, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type71 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type71{sub: Default::default(), e: Default::default(), f: F71::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type71 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E72 { A, B, C }
impl Default for E72 { fn default() -> E72 { E72::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F72 { A, B, C }
impl Default for F72 { fn default() -> F72 { F72::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub72 { pub e: E72, pub f: F72, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub72 { pub fn new(name: String) -> Sub72 { Sub72{e: Default::default(), f: F72::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type72 { pub sub: Sub72, pub e: E72, pub f: F72, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type72 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type72{sub: Default::default(), e: Default::default(), f: F72::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type72 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E73 { A, B, C }
impl Default for E73 { fn default() -> E73 { E73::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F73 { A, B, C }
impl Default for F73 { fn default() -> F73 { F73::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub73 { pub e: E73, pub f: F73, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub73 { pub fn new(name: String) -> Sub73 { Sub73{e: Default::default(), f: F73::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type73 { pub sub: Sub73, pub e: E73, pub f: F73, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type73 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type73{sub: Default::default(), e: Default::default(), f: F73::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type73 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E74 { A, B, C }
impl Default for E74 { fn default() -> E74 { E74::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F74 { A, B, C }
impl Default for F74 { fn default() -> F74 { F74::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub74 { pub e: E74, pub f: F74, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub74 { pub fn new(name: String) -> Sub74 { Sub74{e: Default::default(), f: F74::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type74 { pub sub: Sub74, pub e: E74, pub f: F74, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type74 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type74{sub: Default::default(), e: Default::default(), f: F74::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type74 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E75 { A, B, C }
impl Default for E75 { fn default() -> E75 { E75::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F75 { A, B, C }
impl Default for F75 { fn default() -> F75 { F75::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub75 { pub e: E75, pub f: F75, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub75 { pub fn new(name: String) -> Sub75 { Sub75{e: Default::default(), f: F75::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type75 { pub sub: Sub75, pub e: E75, pub f: F75, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type75 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type75{sub: Default::default(), e: Default::default(), f: F75::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type75 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E76 { A, B, C }
impl Default for E76 { fn default() -> E76 { E76::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F76 { A, B, C }
impl Default for F76 { fn default() -> F76 { F76::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub76 { pub e: E76, pub f: F76, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub76 { pub fn new(name: String) -> Sub76 { Sub76{e: Default::default(), f: F76::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type76 { pub sub: Sub76, pub e: E76, pub f: F76, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type76 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type76{sub: Default::default(), e: Default::default(), f: F76::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type76 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E77 { A, B, C }
impl Default for E77 { fn default() -> E77 { E77::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F77 { A, B, C }
impl Default for F77 { fn default() -> F77 { F77::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub77 { pub e: E77, pub f: F77, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub77 { pub fn new(name: String) -> Sub77 { Sub77{e: Default::default(), f: F77::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type77 { pub sub: Sub77, pub e: E77, pub f: F77, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type77 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type77{sub: Default::default(), e: Default::default(), f: F77::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type77 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E78 { A, B, C }
impl Default for E78 { fn default() -> E78 { E78::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F78 { A, B, C }
impl Default for F78 { fn default() -> F78 { F78::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub78 { pub e: E78, pub f: F78, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub78 { pub fn new(name: String) -> Sub78 { Sub78{e: Default::default(), f: F78::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type78 { pub sub: Sub78, pub e: E78, pub f: F78, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type78 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type78{sub: Default::default(), e: Default::default(), f: F78::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type78 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E79 { A, B, C }
impl Default for E79 { fn default() -> E79 { E79::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F79 { A, B, C }
impl Default for F79 { fn default() -> F79 { F79::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub79 { pub e: E79, pub f: F79, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub79 { pub fn new(name: String) -> Sub79 { Sub79{e: Default::default(), f: F79::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type79 { pub sub: Sub79, pub e: E79, pub f: F79, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type79 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type79{sub: Default::default(), e: Default::default(), f: F79::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type79 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E80 { A, B, C }
impl Default for E80 { fn default() -> E80 { E80::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F80 { A, B, C }
impl Default for F80 { fn default() -> F80 { F80::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub80 { pub e: E80, pub f: F80, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub80 { pub fn new(name: String) -> Sub80 { Sub80{e: Default::default(), f: F80::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type80 { pub sub: Sub80, pub e: E80, pub f: F80, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type80 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type80{sub: Default::default(), e: Default::default(), f: F80::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type80 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E81 { A, B, C }
impl Default for E81 { fn default() -> E81 { E81::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F81 { A, B, C }
impl Default for F81 { fn default() -> F81 { F81::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub81 { pub e: E81, pub f: F81, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub81 { pub fn new(name: String) -> Sub81 { Sub81{e: Default::default(), f: F81::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type81 { pub sub: Sub81, pub e: E81, pub f: F81, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type81 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type81{sub: Default::default(), e: Default::default(), f: F81::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type81 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E82 { A, B, C }
impl Default for E82 { fn default() -> E82 { E82::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F82 { A, B, C }
impl Default for F82 { fn default() -> F82 { F82::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub82 { pub e: E82, pub f: F82, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub82 { pub fn new(name: String) -> Sub82 { Sub82{e: Default::default(), f: F82::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type82 { pub sub: Sub82, pub e: E82, pub f: F82, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type82 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type82{sub: Default::default(), e: Default::default(), f: F82::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type82 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E83 { A, B, C }
impl Default for E83 { fn default() -> E83 { E83::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F83 { A, B, C }
impl Default for F83 { fn default() -> F83 { F83::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub83 { pub e: E83, pub f: F83, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub83 { pub fn new(name: String) -> Sub83 { Sub83{e: Default::default(), f: F83::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type83 { pub sub: Sub83, pub e: E83, pub f: F83, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type83 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type83{sub: Default::default(), e: Default::default(), f: F83::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type83 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E84 { A, B, C }
impl Default for E84 { fn default() -> E84 { E84::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F84 { A, B, C }
impl Default for F84 { fn default() -> F84 { F84::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub84 { pub e: E84, pub f: F84, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub84 { pub fn new(name: String) -> Sub84 { Sub84{e: Default::default(), f: F84::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type84 { pub sub: Sub84, pub e: E84, pub f: F84, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type84 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type84{sub: Default::default(), e: Default::default(), f: F84::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type84 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E85 { A, B, C }
impl Default for E85 { fn default() -> E85 { E85::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F85 { A, B, C }
impl Default for F85 { fn default() -> F85 { F85::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub85 { pub e: E85, pub f: F85, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub85 { pub fn new(name: String) -> Sub85 { Sub85{e: Default::default(), f: F85::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type85 { pub sub: Sub85, pub e: E85, pub f: F85, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type85 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type85{sub: Default::default(), e: Default::default(), f: F85::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type85 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E86 { A, B, C }
impl Default for E86 { fn default() -> E86 { E86::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F86 { A, B, C }
impl Default for F86 { fn default() -> F86 { F86::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub86 { pub e: E86, pub f: F86, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub86 { pub fn new(name: String) -> Sub86 { Sub86{e: Default::default(), f: F86::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type86 { pub sub: Sub86, pub e: E86, pub f: F86, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type86 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type86{sub: Default::default(), e: Default::default(), f: F86::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type86 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E87 { A, B, C }
impl Default for E87 { fn default() -> E87 { E87::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F87 { A, B, C }
impl Default for F87 { fn default() -> F87 { F87::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub87 { pub e: E87, pub f: F87, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub87 { pub fn new(name: String) -> Sub87 { Sub87{e: Default::default(), f: F87::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type87 { pub sub: Sub87, pub e: E87, pub f: F87, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type87 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type87{sub: Default::default(), e: Default::default(), f: F87::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type87 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E88 { A, B, C }
impl Default for E88 { fn default() -> E88 { E88::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F88 { A, B, C }
impl Default for F88 { fn default() -> F88 { F88::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub88 { pub e: E88, pub f: F88, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub88 { pub fn new(name: String) -> Sub88 { Sub88{e: Default::default(), f: F88::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type88 { pub sub: Sub88, pub e: E88, pub f: F88, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type88 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type88{sub: Default::default(), e: Default::default(), f: F88::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type88 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E89 { A, B, C }
impl Default for E89 { fn default() -> E89 { E89::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F89 { A, B, C }
impl Default for F89 { fn default() -> F89 { F89::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub89 { pub e: E89, pub f: F89, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub89 { pub fn new(name: String) -> Sub89 { Sub89{e: Default::default(), f: F89::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type89 { pub sub: Sub89, pub e: E89, pub f: F89, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type89 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type89{sub: Default::default(), e: Default::default(), f: F89::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type89 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E90 { A, B, C }
impl Default for E90 { fn default() -> E90 { E90::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F90 { A, B, C }
impl Default for F90 { fn default() -> F90 { F90::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub90 { pub e: E90, pub f: F90, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub90 { pub fn new(name: String) -> Sub90 { Sub90{e: Default::default(), f: F90::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type90 { pub sub: Sub90, pub e: E90, pub f: F90, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type90 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type90{sub: Default::default(), e: Default::default(), f: F90::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type90 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E91 { A, B, C }
impl Default for E91 { fn default() -> E91 { E91::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F91 { A, B, C }
impl Default for F91 { fn default() -> F91 { F91::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub91 { pub e: E91, pub f: F91, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub91 { pub fn new(name: String) -> Sub91 { Sub91{e: Default::default(), f: F91::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type91 { pub sub: Sub91, pub e: E91, pub f: F91, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type91 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type91{sub: Default::default(), e: Default::default(), f: F91::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type91 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E92 { A, B, C }
impl Default for E92 { fn default() -> E92 { E92::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F92 { A, B, C }
impl Default for F92 { fn default() -> F92 { F92::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub92 { pub e: E92, pub f: F92, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub92 { pub fn new(name: String) -> Sub92 { Sub92{e: Default::default(), f: F92::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type92 { pub sub: Sub92, pub e: E92, pub f: F92, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type92 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type92{sub: Default::default(), e: Default::default(), f: F92::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type92 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E93 { A, B, C }
impl Default for E93 { fn default() -> E93 { E93::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F93 { A, B, C }
impl Default for F93 { fn default() -> F93 { F93::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub93 { pub e: E93, pub f: F93, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub93 { pub fn new(name: String) -> Sub93 { Sub93{e: Default::default(), f: F93::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type93 { pub sub: Sub93, pub e: E93, pub f: F93, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type93 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type93{sub: Default::default(), e: Default::default(), f: F93::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type93 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E94 { A, B, C }
impl Default for E94 { fn default() -> E94 { E94::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F94 { A, B, C }
impl Default for F94 { fn default() -> F94 { F94::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub94 { pub e: E94, pub f: F94, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub94 { pub fn new(name: String) -> Sub94 { Sub94{e: Default::default(), f: F94::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type94 { pub sub: Sub94, pub e: E94, pub f: F94, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type94 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type94{sub: Default::default(), e: Default::default(), f: F94::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type94 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E95 { A, B, C }
impl Default for E95 { fn default() -> E95 { E95::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F95 { A, B, C }
impl Default for F95 { fn default() -> F95 { F95::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub95 { pub e: E95, pub f: F95, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub95 { pub fn new(name: String) -> Sub95 { Sub95{e: Default::default(), f: F95::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type95 { pub sub: Sub95, pub e: E95, pub f: F95, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type95 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type95{sub: Default::default(), e: Default::default(), f: F95::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type95 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E96 { A, B, C }
impl Default for E96 { fn default() -> E96 { E96::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F96 { A, B, C }
impl Default for F96 { fn default() -> F96 { F96::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub96 { pub e: E96, pub f: F96, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub96 { pub fn new(name: String) -> Sub96 { Sub96{e: Default::default(), f: F96::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type96 { pub sub: Sub96, pub e: E96, pub f: F96, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type96 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type96{sub: Default::default(), e: Default::default(), f: F96::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type96 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E97 { A, B, C }
impl Default for E97 { fn default() -> E97 { E97::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F97 { A, B, C }
impl Default for F97 { fn default() -> F97 { F97::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub97 { pub e: E97, pub f: F97, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub97 { pub fn new(name: String) -> Sub97 { Sub97{e: Default::default(), f: F97::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type97 { pub sub: Sub97, pub e: E97, pub f: F97, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type97 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type97{sub: Default::default(), e: Default::default(), f: F97::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type97 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E98 { A, B, C }
impl Default for E98 { fn default() -> E98 { E98::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F98 { A, B, C }
impl Default for F98 { fn default() -> F98 { F98::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub98 { pub e: E98, pub f: F98, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub98 { pub fn new(name: String) -> Sub98 { Sub98{e: Default::default(), f: F98::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type98 { pub sub: Sub98, pub e: E98, pub f: F98, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type98 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type98{sub: Default::default(), e: Default::default(), f: F98::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type98 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E99 { A, B, C }
impl Default for E99 { fn default() -> E99 { E99::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F99 { A, B, C }
impl Default for F99 { fn default() -> F99 { F99::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub99 { pub e: E99, pub f: F99, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub99 { pub fn new(name: String) -> Sub99 { Sub99{e: Default::default(), f: F99::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type99 { pub sub: Sub99, pub e: E99, pub f: F99, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type99 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type99{sub: Default::default(), e: Default::default(), f: F99::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type99 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E100 { A, B, C }
impl Default for E100 { fn default() -> E100 { E100::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F100 { A, B, C }
impl Default for F100 { fn default() -> F100 { F100::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub100 { pub e: E100, pub f: F100, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub100 { pub fn new(name: String) -> Sub100 { Sub100{e: Default::default(), f: F100::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type100 { pub sub: Sub100, pub e: E100, pub f: F100, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type100 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type100{sub: Default::default(), e: Default::default(), f: F100::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type100 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E101 { A, B, C }
impl Default for E101 { fn default() -> E101 { E101::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F101 { A, B, C }
impl Default for F101 { fn default() -> F101 { F101::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub101 { pub e: E101, pub f: F101, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub101 { pub fn new(name: String) -> Sub101 { Sub101{e: Default::default(), f: F101::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type101 { pub sub: Sub101, pub e: E101, pub f: F101, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type101 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type101{sub: Default::default(), e: Default::default(), f: F101::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type101 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E102 { A, B, C }
impl Default for E102 { fn default() -> E102 { E102::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F102 { A, B, C }
impl Default for F102 { fn default() -> F102 { F102::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub102 { pub e: E102, pub f: F102, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub102 { pub fn new(name: String) -> Sub102 { Sub102{e: Default::default(), f: F102::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type102 { pub sub: Sub102, pub e: E102, pub f: F102, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type102 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type102{sub: Default::default(), e: Default::default(), f: F102::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type102 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E103 { A, B, C }
impl Default for E103 { fn default() -> E103 { E103::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F103 { A, B, C }
impl Default for F103 { fn default() -> F103 { F103::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub103 { pub e: E103, pub f: F103, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub103 { pub fn new(name: String) -> Sub103 { Sub103{e: Default::default(), f: F103::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type103 { pub sub: Sub103, pub e: E103, pub f: F103, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type103 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type103{sub: Default::default(), e: Default::default(), f: F103::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type103 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E104 { A, B, C }
impl Default for E104 { fn default() -> E104 { E104::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F104 { A, B, C }
impl Default for F104 { fn default() -> F104 { F104::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub104 { pub e: E104, pub f: F104, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub104 { pub fn new(name: String) -> Sub104 { Sub104{e: Default::default(), f: F104::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type104 { pub sub: Sub104, pub e: E104, pub f: F104, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type104 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type104{sub: Default::default(), e: Default::default(), f: F104::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type104 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E105 { A, B, C }
impl Default for E105 { fn default() -> E105 { E105::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F105 { A, B, C }
impl Default for F105 { fn default() -> F105 { F105::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub105 { pub e: E105, pub f: F105, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub105 { pub fn new(name: String) -> Sub105 { Sub105{e: Default::default(), f: F105::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type105 { pub sub: Sub105, pub e: E105, pub f: F105, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type105 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type105{sub: Default::default(), e: Default::default(), f: F105::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type105 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E106 { A, B, C }
impl Default for E106 { fn default() -> E106 { E106::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F106 { A, B, C }
impl Default for F106 { fn default() -> F106 { F106::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub106 { pub e: E106, pub f: F106, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub106 { pub fn new(name: String) -> Sub106 { Sub106{e: Default::default(), f: F106::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type106 { pub sub: Sub106, pub e: E106, pub f: F106, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type106 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type106{sub: Default::default(), e: Default::default(), f: F106::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type106 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E107 { A, B, C }
impl Default for E107 { fn default() -> E107 { E107::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F107 { A, B, C }
impl Default for F107 { fn default() -> F107 { F107::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub107 { pub e: E107, pub f: F107, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub107 { pub fn new(name: String) -> Sub107 { Sub107{e: Default::default(), f: F107::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type107 { pub sub: Sub107, pub e: E107, pub f: F107, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type107 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type107{sub: Default::default(), e: Default::default(), f: F107::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type107 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E108 { A, B, C }
impl Default for E108 { fn default() -> E108 { E108::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F108 { A, B, C }
impl Default for F108 { fn default() -> F108 { F108::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub108 { pub e: E108, pub f: F108, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub108 { pub fn new(name: String) -> Sub108 { Sub108{e: Default::default(), f: F108::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type108 { pub sub: Sub108, pub e: E108, pub f: F108, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type108 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type108{sub: Default::default(), e: Default::default(), f: F108::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type108 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E109 { A, B, C }
impl Default for E109 { fn default() -> E109 { E109::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F109 { A, B, C }
impl Default for F109 { fn default() -> F109 { F109::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub109 { pub e: E109, pub f: F109, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub109 { pub fn new(name: String) -> Sub109 { Sub109{e: Default::default(), f: F109::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type109 { pub sub: Sub109, pub e: E109, pub f: F109, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type109 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type109{sub: Default::default(), e: Default::default(), f: F109::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type109 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E110 { A, B, C }
impl Default for E110 { fn default() -> E110 { E110::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F110 { A, B, C }
impl Default for F110 { fn default() -> F110 { F110::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub110 { pub e: E110, pub f: F110, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub110 { pub fn new(name: String) -> Sub110 { Sub110{e: Default::default(), f: F110::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type110 { pub sub: Sub110, pub e: E110, pub f: F110, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type110 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type110{sub: Default::default(), e: Default::default(), f: F110::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type110 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E111 { A, B, C }
impl Default for E111 { fn default() -> E111 { E111::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F111 { A, B, C }
impl Default for F111 { fn default() -> F111 { F111::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub111 { pub e: E111, pub f: F111, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub111 { pub fn new(name: String) -> Sub111 { Sub111{e: Default::default(), f: F111::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type111 { pub sub: Sub111, pub e: E111, pub f: F111, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type111 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type111{sub: Default::default(), e: Default::default(), f: F111::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type111 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E112 { A, B, C }
impl Default for E112 { fn default() -> E112 { E112::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F112 { A, B, C }
impl Default for F112 { fn default() -> F112 { F112::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub112 { pub e: E112, pub f: F112, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub112 { pub fn new(name: String) -> Sub112 { Sub112{e: Default::default(), f: F112::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type112 { pub sub: Sub112, pub e: E112, pub f: F112, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type112 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type112{sub: Default::default(), e: Default::default(), f: F112::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type112 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E113 { A, B, C }
impl Default for E113 { fn default() -> E113 { E113::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F113 { A, B, C }
impl Default for F113 { fn default() -> F113 { F113::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub113 { pub e: E113, pub f: F113, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub113 { pub fn new(name: String) -> Sub113 { Sub113{e: Default::default(), f: F113::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type113 { pub sub: Sub113, pub e: E113, pub f: F113, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type113 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type113{sub: Default::default(), e: Default::default(), f: F113::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type113 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E114 { A, B, C }
impl Default for E114 { fn default() -> E114 { E114::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F114 { A, B, C }
impl Default for F114 { fn default() -> F114 { F114::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub114 { pub e: E114, pub f: F114, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub114 { pub fn new(name: String) -> Sub114 { Sub114{e: Default::default(), f: F114::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type114 { pub sub: Sub114, pub e: E114, pub f: F114, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type114 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type114{sub: Default::default(), e: Default::default(), f: F114::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type114 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E115 { A, B, C }
impl Default for E115 { fn default() -> E115 { E115::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F115 { A, B, C }
impl Default for F115 { fn default() -> F115 { F115::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub115 { pub e: E115, pub f: F115, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub115 { pub fn new(name: String) -> Sub115 { Sub115{e: Default::default(), f: F115::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type115 { pub sub: Sub115, pub e: E115, pub f: F115, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type115 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type115{sub: Default::default(), e: Default::default(), f: F115::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type115 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E116 { A, B, C }
impl Default for E116 { fn default() -> E116 { E116::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F116 { A, B, C }
impl Default for F116 { fn default() -> F116 { F116::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub116 { pub e: E116, pub f: F116, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub116 { pub fn new(name: String) -> Sub116 { Sub116{e: Default::default(), f: F116::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type116 { pub sub: Sub116, pub e: E116, pub f: F116, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type116 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type116{sub: Default::default(), e: Default::default(), f: F116::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type116 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E117 { A, B, C }
impl Default for E117 { fn default() -> E117 { E117::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F117 { A, B, C }
impl Default for F117 { fn default() -> F117 { F117::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub117 { pub e: E117, pub f: F117, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub117 { pub fn new(name: String) -> Sub117 { Sub117{e: Default::default(), f: F117::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type117 { pub sub: Sub117, pub e: E117, pub f: F117, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type117 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type117{sub: Default::default(), e: Default::default(), f: F117::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type117 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E118 { A, B, C }
impl Default for E118 { fn default() -> E118 { E118::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F118 { A, B, C }
impl Default for F118 { fn default() -> F118 { F118::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub118 { pub e: E118, pub f: F118, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub118 { pub fn new(name: String) -> Sub118 { Sub118{e: Default::default(), f: F118::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type118 { pub sub: Sub118, pub e: E118, pub f: F118, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type118 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type118{sub: Default::default(), e: Default::default(), f: F118::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type118 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E119 { A, B, C }
impl Default for E119 { fn default() -> E119 { E119::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F119 { A, B, C }
impl Default for F119 { fn default() -> F119 { F119::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub119 { pub e: E119, pub f: F119, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub119 { pub fn new(name: String) -> Sub119 { Sub119{e: Default::default(), f: F119::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type119 { pub sub: Sub119, pub e: E119, pub f: F119, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type119 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type119{sub: Default::default(), e: Default::default(), f: F119::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type119 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E120 { A, B, C }
impl Default for E120 { fn default() -> E120 { E120::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F120 { A, B, C }
impl Default for F120 { fn default() -> F120 { F120::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub120 { pub e: E120, pub f: F120, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub120 { pub fn new(name: String) -> Sub120 { Sub120{e: Default::default(), f: F120::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type120 { pub sub: Sub120, pub e: E120, pub f: F120, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type120 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type120{sub: Default::default(), e: Default::default(), f: F120::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type120 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E121 { A, B, C }
impl Default for E121 { fn default() -> E121 { E121::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F121 { A, B, C }
impl Default for F121 { fn default() -> F121 { F121::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub121 { pub e: E121, pub f: F121, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub121 { pub fn new(name: String) -> Sub121 { Sub121{e: Default::default(), f: F121::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type121 { pub sub: Sub121, pub e: E121, pub f: F121, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type121 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type121{sub: Default::default(), e: Default::default(), f: F121::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type121 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E122 { A, B, C }
impl Default for E122 { fn default() -> E122 { E122::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F122 { A, B, C }
impl Default for F122 { fn default() -> F122 { F122::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub122 { pub e: E122, pub f: F122, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub122 { pub fn new(name: String) -> Sub122 { Sub122{e: Default::default(), f: F122::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type122 { pub sub: Sub122, pub e: E122, pub f: F122, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type122 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type122{sub: Default::default(), e: Default::default(), f: F122::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type122 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E123 { A, B, C }
impl Default for E123 { fn default() -> E123 { E123::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F123 { A, B, C }
impl Default for F123 { fn default() -> F123 { F123::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub123 { pub e: E123, pub f: F123, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub123 { pub fn new(name: String) -> Sub123 { Sub123{e: Default::default(), f: F123::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type123 { pub sub: Sub123, pub e: E123, pub f: F123, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type123 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type123{sub: Default::default(), e: Default::default(), f: F123::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type123 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E124 { A, B, C }
impl Default for E124 { fn default() -> E124 { E124::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F124 { A, B, C }
impl Default for F124 { fn default() -> F124 { F124::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub124 { pub e: E124, pub f: F124, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub124 { pub fn new(name: String) -> Sub124 { Sub124{e: Default::default(), f: F124::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type124 { pub sub: Sub124, pub e: E124, pub f: F124, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type124 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type124{sub: Default::default(), e: Default::default(), f: F124::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type124 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E125 { A, B, C }
impl Default for E125 { fn default() -> E125 { E125::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F125 { A, B, C }
impl Default for F125 { fn default() -> F125 { F125::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub125 { pub e: E125, pub f: F125, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub125 { pub fn new(name: String) -> Sub125 { Sub125{e: Default::default(), f: F125::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type125 { pub sub: Sub125, pub e: E125, pub f: F125, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type125 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type125{sub: Default::default(), e: Default::default(), f: F125::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type125 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E126 { A, B, C }
impl Default for E126 { fn default() -> E126 { E126::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F126 { A, B, C }
impl Default for F126 { fn default() -> F126 { F126::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub126 { pub e: E126, pub f: F126, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub126 { pub fn new(name: String) -> Sub126 { Sub126{e: Default::default(), f: F126::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type126 { pub sub: Sub126, pub e: E126, pub f: F126, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type126 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type126{sub: Default::default(), e: Default::default(), f: F126::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type126 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E127 { A, B, C }
impl Default for E127 { fn default() -> E127 { E127::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F127 { A, B, C }
impl Default for F127 { fn default() -> F127 { F127::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub127 { pub e: E127, pub f: F127, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub127 { pub fn new(name: String) -> Sub127 { Sub127{e: Default::default(), f: F127::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type127 { pub sub: Sub127, pub e: E127, pub f: F127, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type127 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type127{sub: Default::default(), e: Default::default(), f: F127::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type127 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E128 { A, B, C }
impl Default for E128 { fn default() -> E128 { E128::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F128 { A, B, C }
impl Default for F128 { fn default() -> F128 { F128::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub128 { pub e: E128, pub f: F128, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub128 { pub fn new(name: String) -> Sub128 { Sub128{e: Default::default(), f: F128::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type128 { pub sub: Sub128, pub e: E128, pub f: F128, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type128 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type128{sub: Default::default(), e: Default::default(), f: F128::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type128 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E129 { A, B, C }
impl Default for E129 { fn default() -> E129 { E129::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F129 { A, B, C }
impl Default for F129 { fn default() -> F129 { F129::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub129 { pub e: E129, pub f: F129, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub129 { pub fn new(name: String) -> Sub129 { Sub129{e: Default::default(), f: F129::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type129 { pub sub: Sub129, pub e: E129, pub f: F129, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type129 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type129{sub: Default::default(), e: Default::default(), f: F129::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type129 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E130 { A, B, C }
impl Default for E130 { fn default() -> E130 { E130::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F130 { A, B, C }
impl Default for F130 { fn default() -> F130 { F130::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub130 { pub e: E130, pub f: F130, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub130 { pub fn new(name: String) -> Sub130 { Sub130{e: Default::default(), f: F130::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type130 { pub sub: Sub130, pub e: E130, pub f: F130, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type130 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type130{sub: Default::default(), e: Default::default(), f: F130::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type130 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E131 { A, B, C }
impl Default for E131 { fn default() -> E131 { E131::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F131 { A, B, C }
impl Default for F131 { fn default() -> F131 { F131::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub131 { pub e: E131, pub f: F131, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub131 { pub fn new(name: String) -> Sub131 { Sub131{e: Default::default(), f: F131::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type131 { pub sub: Sub131, pub e: E131, pub f: F131, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type131 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type131{sub: Default::default(), e: Default::default(), f: F131::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type131 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E132 { A, B, C }
impl Default for E132 { fn default() -> E132 { E132::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F132 { A, B, C }
impl Default for F132 { fn default() -> F132 { F132::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub132 { pub e: E132, pub f: F132, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub132 { pub fn new(name: String) -> Sub132 { Sub132{e: Default::default(), f: F132::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type132 { pub sub: Sub132, pub e: E132, pub f: F132, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type132 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type132{sub: Default::default(), e: Default::default(), f: F132::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type132 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E133 { A, B, C }
impl Default for E133 { fn default() -> E133 { E133::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F133 { A, B, C }
impl Default for F133 { fn default() -> F133 { F133::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub133 { pub e: E133, pub f: F133, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub133 { pub fn new(name: String) -> Sub133 { Sub133{e: Default::default(), f: F133::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type133 { pub sub: Sub133, pub e: E133, pub f: F133, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type133 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type133{sub: Default::default(), e: Default::default(), f: F133::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type133 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E134 { A, B, C }
impl Default for E134 { fn default() -> E134 { E134::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F134 { A, B, C }
impl Default for F134 { fn default() -> F134 { F134::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub134 { pub e: E134, pub f: F134, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub134 { pub fn new(name: String) -> Sub134 { Sub134{e: Default::default(), f: F134::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type134 { pub sub: Sub134, pub e: E134, pub f: F134, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type134 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type134{sub: Default::default(), e: Default::default(), f: F134::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type134 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E135 { A, B, C }
impl Default for E135 { fn default() -> E135 { E135::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F135 { A, B, C }
impl Default for F135 { fn default() -> F135 { F135::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub135 { pub e: E135, pub f: F135, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub135 { pub fn new(name: String) -> Sub135 { Sub135{e: Default::default(), f: F135::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type135 { pub sub: Sub135, pub e: E135, pub f: F135, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type135 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type135{sub: Default::default(), e: Default::default(), f: F135::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type135 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E136 { A, B, C }
impl Default for E136 { fn default() -> E136 { E136::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F136 { A, B, C }
impl Default for F136 { fn default() -> F136 { F136::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub136 { pub e: E136, pub f: F136, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub136 { pub fn new(name: String) -> Sub136 { Sub136{e: Default::default(), f: F136::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type136 { pub sub: Sub136, pub e: E136, pub f: F136, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type136 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type136{sub: Default::default(), e: Default::default(), f: F136::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type136 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E137 { A, B, C }
impl Default for E137 { fn default() -> E137 { E137::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F137 { A, B, C }
impl Default for F137 { fn default() -> F137 { F137::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub137 { pub e: E137, pub f: F137, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub137 { pub fn new(name: String) -> Sub137 { Sub137{e: Default::default(), f: F137::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type137 { pub sub: Sub137, pub e: E137, pub f: F137, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type137 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type137{sub: Default::default(), e: Default::default(), f: F137::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type137 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E138 { A, B, C }
impl Default for E138 { fn default() -> E138 { E138::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F138 { A, B, C }
impl Default for F138 { fn default() -> F138 { F138::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub138 { pub e: E138, pub f: F138, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub138 { pub fn new(name: String) -> Sub138 { Sub138{e: Default::default(), f: F138::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type138 { pub sub: Sub138, pub e: E138, pub f: F138, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type138 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type138{sub: Default::default(), e: Default::default(), f: F138::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type138 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E139 { A, B, C }
impl Default for E139 { fn default() -> E139 { E139::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F139 { A, B, C }
impl Default for F139 { fn default() -> F139 { F139::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub139 { pub e: E139, pub f: F139, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub139 { pub fn new(name: String) -> Sub139 { Sub139{e: Default::default(), f: F139::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type139 { pub sub: Sub139, pub e: E139, pub f: F139, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type139 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type139{sub: Default::default(), e: Default::default(), f: F139::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type139 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E140 { A, B, C }
impl Default for E140 { fn default() -> E140 { E140::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F140 { A, B, C }
impl Default for F140 { fn default() -> F140 { F140::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub140 { pub e: E140, pub f: F140, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub140 { pub fn new(name: String) -> Sub140 { Sub140{e: Default::default(), f: F140::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type140 { pub sub: Sub140, pub e: E140, pub f: F140, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type140 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type140{sub: Default::default(), e: Default::default(), f: F140::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type140 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E141 { A, B, C }
impl Default for E141 { fn default() -> E141 { E141::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F141 { A, B, C }
impl Default for F141 { fn default() -> F141 { F141::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub141 { pub e: E141, pub f: F141, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub141 { pub fn new(name: String) -> Sub141 { Sub141{e: Default::default(), f: F141::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type141 { pub sub: Sub141, pub e: E141, pub f: F141, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type141 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type141{sub: Default::default(), e: Default::default(), f: F141::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type141 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E142 { A, B, C }
impl Default for E142 { fn default() -> E142 { E142::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F142 { A, B, C }
impl Default for F142 { fn default() -> F142 { F142::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub142 { pub e: E142, pub f: F142, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub142 { pub fn new(name: String) -> Sub142 { Sub142{e: Default::default(), f: F142::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type142 { pub sub: Sub142, pub e: E142, pub f: F142, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type142 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type142{sub: Default::default(), e: Default::default(), f: F142::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type142 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E143 { A, B, C }
impl Default for E143 { fn default() -> E143 { E143::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F143 { A, B, C }
impl Default for F143 { fn default() -> F143 { F143::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub143 { pub e: E143, pub f: F143, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub143 { pub fn new(name: String) -> Sub143 { Sub143{e: Default::default(), f: F143::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type143 { pub sub: Sub143, pub e: E143, pub f: F143, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type143 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type143{sub: Default::default(), e: Default::default(), f: F143::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type143 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E144 { A, B, C }
impl Default for E144 { fn default() -> E144 { E144::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F144 { A, B, C }
impl Default for F144 { fn default() -> F144 { F144::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub144 { pub e: E144, pub f: F144, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub144 { pub fn new(name: String) -> Sub144 { Sub144{e: Default::default(), f: F144::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type144 { pub sub: Sub144, pub e: E144, pub f: F144, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type144 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type144{sub: Default::default(), e: Default::default(), f: F144::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type144 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E145 { A, B, C }
impl Default for E145 { fn default() -> E145 { E145::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F145 { A, B, C }
impl Default for F145 { fn default() -> F145 { F145::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub145 { pub e: E145, pub f: F145, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub145 { pub fn new(name: String) -> Sub145 { Sub145{e: Default::default(), f: F145::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type145 { pub sub: Sub145, pub e: E145, pub f: F145, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type145 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type145{sub: Default::default(), e: Default::default(), f: F145::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type145 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E146 { A, B, C }
impl Default for E146 { fn default() -> E146 { E146::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F146 { A, B, C }
impl Default for F146 { fn default() -> F146 { F146::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub146 { pub e: E146, pub f: F146, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub146 { pub fn new(name: String) -> Sub146 { Sub146{e: Default::default(), f: F146::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type146 { pub sub: Sub146, pub e: E146, pub f: F146, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type146 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type146{sub: Default::default(), e: Default::default(), f: F146::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type146 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E147 { A, B, C }
impl Default for E147 { fn default() -> E147 { E147::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F147 { A, B, C }
impl Default for F147 { fn default() -> F147 { F147::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub147 { pub e: E147, pub f: F147, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub147 { pub fn new(name: String) -> Sub147 { Sub147{e: Default::default(), f: F147::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type147 { pub sub: Sub147, pub e: E147, pub f: F147, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type147 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type147{sub: Default::default(), e: Default::default(), f: F147::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type147 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E148 { A, B, C }
impl Default for E148 { fn default() -> E148 { E148::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F148 { A, B, C }
impl Default for F148 { fn default() -> F148 { F148::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub148 { pub e: E148, pub f: F148, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub148 { pub fn new(name: String) -> Sub148 { Sub148{e: Default::default(), f: F148::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type148 { pub sub: Sub148, pub e: E148, pub f: F148, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type148 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type148{sub: Default::default(), e: Default::default(), f: F148::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type148 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E149 { A, B, C }
impl Default for E149 { fn default() -> E149 { E149::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F149 { A, B, C }
impl Default for F149 { fn default() -> F149 { F149::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub149 { pub e: E149, pub f: F149, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub149 { pub fn new(name: String) -> Sub149 { Sub149{e: Default::default(), f: F149::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type149 { pub sub: Sub149, pub e: E149, pub f: F149, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type149 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type149{sub: Default::default(), e: Default::default(), f: F149::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type149 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E150 { A, B, C }
impl Default for E150 { fn default() -> E150 { E150::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F150 { A, B, C }
impl Default for F150 { fn default() -> F150 { F150::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub150 { pub e: E150, pub f: F150, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub150 { pub fn new(name: String) -> Sub150 { Sub150{e: Default::default(), f: F150::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type150 { pub sub: Sub150, pub e: E150, pub f: F150, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type150 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type150{sub: Default::default(), e: Default::default(), f: F150::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type150 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E151 { A, B, C }
impl Default for E151 { fn default() -> E151 { E151::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F151 { A, B, C }
impl Default for F151 { fn default() -> F151 { F151::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub151 { pub e: E151, pub f: F151, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub151 { pub fn new(name: String) -> Sub151 { Sub151{e: Default::default(), f: F151::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type151 { pub sub: Sub151, pub e: E151, pub f: F151, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type151 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type151{sub: Default::default(), e: Default::default(), f: F151::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type151 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E152 { A, B, C }
impl Default for E152 { fn default() -> E152 { E152::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F152 { A, B, C }
impl Default for F152 { fn default() -> F152 { F152::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub152 { pub e: E152, pub f: F152, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub152 { pub fn new(name: String) -> Sub152 { Sub152{e: Default::default(), f: F152::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type152 { pub sub: Sub152, pub e: E152, pub f: F152, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type152 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type152{sub: Default::default(), e: Default::default(), f: F152::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type152 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E153 { A, B, C }
impl Default for E153 { fn default() -> E153 { E153::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F153 { A, B, C }
impl Default for F153 { fn default() -> F153 { F153::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub153 { pub e: E153, pub f: F153, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub153 { pub fn new(name: String) -> Sub153 { Sub153{e: Default::default(), f: F153::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type153 { pub sub: Sub153, pub e: E153, pub f: F153, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type153 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type153{sub: Default::default(), e: Default::default(), f: F153::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type153 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E154 { A, B, C }
impl Default for E154 { fn default() -> E154 { E154::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F154 { A, B, C }
impl Default for F154 { fn default() -> F154 { F154::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub154 { pub e: E154, pub f: F154, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub154 { pub fn new(name: String) -> Sub154 { Sub154{e: Default::default(), f: F154::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type154 { pub sub: Sub154, pub e: E154, pub f: F154, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type154 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type154{sub: Default::default(), e: Default::default(), f: F154::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type154 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E155 { A, B, C }
impl Default for E155 { fn default() -> E155 { E155::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F155 { A, B, C }
impl Default for F155 { fn default() -> F155 { F155::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub155 { pub e: E155, pub f: F155, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub155 { pub fn new(name: String) -> Sub155 { Sub155{e: Default::default(), f: F155::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type155 { pub sub: Sub155, pub e: E155, pub f: F155, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type155 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type155{sub: Default::default(), e: Default::default(), f: F155::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type155 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E156 { A, B, C }
impl Default for E156 { fn default() -> E156 { E156::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F156 { A, B, C }
impl Default for F156 { fn default() -> F156 { F156::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub156 { pub e: E156, pub f: F156, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub156 { pub fn new(name: String) -> Sub156 { Sub156{e: Default::default(), f: F156::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type156 { pub sub: Sub156, pub e: E156, pub f: F156, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type156 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type156{sub: Default::default(), e: Default::default(), f: F156::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type156 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E157 { A, B, C }
impl Default for E157 { fn default() -> E157 { E157::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F157 { A, B, C }
impl Default for F157 { fn default() -> F157 { F157::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub157 { pub e: E157, pub f: F157, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub157 { pub fn new(name: String) -> Sub157 { Sub157{e: Default::default(), f: F157::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type157 { pub sub: Sub157, pub e: E157, pub f: F157, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type157 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type157{sub: Default::default(), e: Default::default(), f: F157::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type157 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E158 { A, B, C }
impl Default for E158 { fn default() -> E158 { E158::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F158 { A, B, C }
impl Default for F158 { fn default() -> F158 { F158::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub158 { pub e: E158, pub f: F158, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub158 { pub fn new(name: String) -> Sub158 { Sub158{e: Default::default(), f: F158::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type158 { pub sub: Sub158, pub e: E158, pub f: F158, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type158 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type158{sub: Default::default(), e: Default::default(), f: F158::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type158 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E159 { A, B, C }
impl Default for E159 { fn default() -> E159 { E159::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F159 { A, B, C }
impl Default for F159 { fn default() -> F159 { F159::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub159 { pub e: E159, pub f: F159, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub159 { pub fn new(name: String) -> Sub159 { Sub159{e: Default::default(), f: F159::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type159 { pub sub: Sub159, pub e: E159, pub f: F159, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type159 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type159{sub: Default::default(), e: Default::default(), f: F159::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type159 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E160 { A, B, C }
impl Default for E160 { fn default() -> E160 { E160::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F160 { A, B, C }
impl Default for F160 { fn default() -> F160 { F160::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub160 { pub e: E160, pub f: F160, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub160 { pub fn new(name: String) -> Sub160 { Sub160{e: Default::default(), f: F160::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type160 { pub sub: Sub160, pub e: E160, pub f: F160, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type160 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type160{sub: Default::default(), e: Default::default(), f: F160::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type160 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E161 { A, B, C }
impl Default for E161 { fn default() -> E161 { E161::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F161 { A, B, C }
impl Default for F161 { fn default() -> F161 { F161::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub161 { pub e: E161, pub f: F161, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub161 { pub fn new(name: String) -> Sub161 { Sub161{e: Default::default(), f: F161::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type161 { pub sub: Sub161, pub e: E161, pub f: F161, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type161 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type161{sub: Default::default(), e: Default::default(), f: F161::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type161 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E162 { A, B, C }
impl Default for E162 { fn default() -> E162 { E162::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F162 { A, B, C }
impl Default for F162 { fn default() -> F162 { F162::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub162 { pub e: E162, pub f: F162, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub162 { pub fn new(name: String) -> Sub162 { Sub162{e: Default::default(), f: F162::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type162 { pub sub: Sub162, pub e: E162, pub f: F162, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type162 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type162{sub: Default::default(), e: Default::default(), f: F162::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type162 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E163 { A, B, C }
impl Default for E163 { fn default() -> E163 { E163::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F163 { A, B, C }
impl Default for F163 { fn default() -> F163 { F163::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub163 { pub e: E163, pub f: F163, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub163 { pub fn new(name: String) -> Sub163 { Sub163{e: Default::default(), f: F163::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type163 { pub sub: Sub163, pub e: E163, pub f: F163, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type163 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type163{sub: Default::default(), e: Default::default(), f: F163::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type163 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E164 { A, B, C }
impl Default for E164 { fn default() -> E164 { E164::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F164 { A, B, C }
impl Default for F164 { fn default() -> F164 { F164::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub164 { pub e: E164, pub f: F164, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub164 { pub fn new(name: String) -> Sub164 { Sub164{e: Default::default(), f: F164::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type164 { pub sub: Sub164, pub e: E164, pub f: F164, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type164 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type164{sub: Default::default(), e: Default::default(), f: F164::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type164 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E165 { A, B, C }
impl Default for E165 { fn default() -> E165 { E165::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F165 { A, B, C }
impl Default for F165 { fn default() -> F165 { F165::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub165 { pub e: E165, pub f: F165, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub165 { pub fn new(name: String) -> Sub165 { Sub165{e: Default::default(), f: F165::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type165 { pub sub: Sub165, pub e: E165, pub f: F165, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type165 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type165{sub: Default::default(), e: Default::default(), f: F165::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type165 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E166 { A, B, C }
impl Default for E166 { fn default() -> E166 { E166::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F166 { A, B, C }
impl Default for F166 { fn default() -> F166 { F166::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub166 { pub e: E166, pub f: F166, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub166 { pub fn new(name: String) -> Sub166 { Sub166{e: Default::default(), f: F166::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type166 { pub sub: Sub166, pub e: E166, pub f: F166, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type166 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type166{sub: Default::default(), e: Default::default(), f: F166::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type166 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E167 { A, B, C }
impl Default for E167 { fn default() -> E167 { E167::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F167 { A, B, C }
impl Default for F167 { fn default() -> F167 { F167::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub167 { pub e: E167, pub f: F167, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub167 { pub fn new(name: String) -> Sub167 { Sub167{e: Default::default(), f: F167::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type167 { pub sub: Sub167, pub e: E167, pub f: F167, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type167 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type167{sub: Default::default(), e: Default::default(), f: F167::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type167 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E168 { A, B, C }
impl Default for E168 { fn default() -> E168 { E168::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F168 { A, B, C }
impl Default for F168 { fn default() -> F168 { F168::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub168 { pub e: E168, pub f: F168, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub168 { pub fn new(name: String) -> Sub168 { Sub168{e: Default::default(), f: F168::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type168 { pub sub: Sub168, pub e: E168, pub f: F168, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type168 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type168{sub: Default::default(), e: Default::default(), f: F168::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type168 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E169 { A, B, C }
impl Default for E169 { fn default() -> E169 { E169::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F169 { A, B, C }
impl Default for F169 { fn default() -> F169 { F169::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub169 { pub e: E169, pub f: F169, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub169 { pub fn new(name: String) -> Sub169 { Sub169{e: Default::default(), f: F169::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type169 { pub sub: Sub169, pub e: E169, pub f: F169, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type169 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type169{sub: Default::default(), e: Default::default(), f: F169::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type169 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E170 { A, B, C }
impl Default for E170 { fn default() -> E170 { E170::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F170 { A, B, C }
impl Default for F170 { fn default() -> F170 { F170::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub170 { pub e: E170, pub f: F170, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub170 { pub fn new(name: String) -> Sub170 { Sub170{e: Default::default(), f: F170::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type170 { pub sub: Sub170, pub e: E170, pub f: F170, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type170 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type170{sub: Default::default(), e: Default::default(), f: F170::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type170 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E171 { A, B, C }
impl Default for E171 { fn default() -> E171 { E171::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F171 { A, B, C }
impl Default for F171 { fn default() -> F171 { F171::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub171 { pub e: E171, pub f: F171, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub171 { pub fn new(name: String) -> Sub171 { Sub171{e: Default::default(), f: F171::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type171 { pub sub: Sub171, pub e: E171, pub f: F171, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type171 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type171{sub: Default::default(), e: Default::default(), f: F171::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type171 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E172 { A, B, C }
impl Default for E172 { fn default() -> E172 { E172::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F172 { A, B, C }
impl Default for F172 { fn default() -> F172 { F172::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub172 { pub e: E172, pub f: F172, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub172 { pub fn new(name: String) -> Sub172 { Sub172{e: Default::default(), f: F172::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type172 { pub sub: Sub172, pub e: E172, pub f: F172, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type172 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type172{sub: Default::default(), e: Default::default(), f: F172::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type172 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E173 { A, B, C }
impl Default for E173 { fn default() -> E173 { E173::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F173 { A, B, C }
impl Default for F173 { fn default() -> F173 { F173::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub173 { pub e: E173, pub f: F173, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub173 { pub fn new(name: String) -> Sub173 { Sub173{e: Default::default(), f: F173::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type173 { pub sub: Sub173, pub e: E173, pub f: F173, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type173 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type173{sub: Default::default(), e: Default::default(), f: F173::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type173 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E174 { A, B, C }
impl Default for E174 { fn default() -> E174 { E174::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F174 { A, B, C }
impl Default for F174 { fn default() -> F174 { F174::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub174 { pub e: E174, pub f: F174, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub174 { pub fn new(name: String) -> Sub174 { Sub174{e: Default::default(), f: F174::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type174 { pub sub: Sub174, pub e: E174, pub f: F174, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type174 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type174{sub: Default::default(), e: Default::default(), f: F174::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type174 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E175 { A, B, C }
impl Default for E175 { fn default() -> E175 { E175::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F175 { A, B, C }
impl Default for F175 { fn default() -> F175 { F175::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub175 { pub e: E175, pub f: F175, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub175 { pub fn new(name: String) -> Sub175 { Sub175{e: Default::default(), f: F175::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type175 { pub sub: Sub175, pub e: E175, pub f: F175, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type175 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type175{sub: Default::default(), e: Default::default(), f: F175::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type175 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E176 { A, B, C }
impl Default for E176 { fn default() -> E176 { E176::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F176 { A, B, C }
impl Default for F176 { fn default() -> F176 { F176::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub176 { pub e: E176, pub f: F176, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub176 { pub fn new(name: String) -> Sub176 { Sub176{e: Default::default(), f: F176::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type176 { pub sub: Sub176, pub e: E176, pub f: F176, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type176 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type176{sub: Default::default(), e: Default::default(), f: F176::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type176 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E177 { A, B, C }
impl Default for E177 { fn default() -> E177 { E177::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F177 { A, B, C }
impl Default for F177 { fn default() -> F177 { F177::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub177 { pub e: E177, pub f: F177, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub177 { pub fn new(name: String) -> Sub177 { Sub177{e: Default::default(), f: F177::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type177 { pub sub: Sub177, pub e: E177, pub f: F177, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type177 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type177{sub: Default::default(), e: Default::default(), f: F177::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type177 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E178 { A, B, C }
impl Default for E178 { fn default() -> E178 { E178::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F178 { A, B, C }
impl Default for F178 { fn default() -> F178 { F178::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub178 { pub e: E178, pub f: F178, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub178 { pub fn new(name: String) -> Sub178 { Sub178{e: Default::default(), f: F178::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type178 { pub sub: Sub178, pub e: E178, pub f: F178, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type178 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type178{sub: Default::default(), e: Default::default(), f: F178::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type178 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E179 { A, B, C }
impl Default for E179 { fn default() -> E179 { E179::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F179 { A, B, C }
impl Default for F179 { fn default() -> F179 { F179::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub179 { pub e: E179, pub f: F179, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub179 { pub fn new(name: String) -> Sub179 { Sub179{e: Default::default(), f: F179::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type179 { pub sub: Sub179, pub e: E179, pub f: F179, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type179 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type179{sub: Default::default(), e: Default::default(), f: F179::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type179 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E180 { A, B, C }
impl Default for E180 { fn default() -> E180 { E180::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F180 { A, B, C }
impl Default for F180 { fn default() -> F180 { F180::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub180 { pub e: E180, pub f: F180, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub180 { pub fn new(name: String) -> Sub180 { Sub180{e: Default::default(), f: F180::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type180 { pub sub: Sub180, pub e: E180, pub f: F180, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type180 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type180{sub: Default::default(), e: Default::default(), f: F180::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type180 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E181 { A, B, C }
impl Default for E181 { fn default() -> E181 { E181::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F181 { A, B, C }
impl Default for F181 { fn default() -> F181 { F181::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub181 { pub e: E181, pub f: F181, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub181 { pub fn new(name: String) -> Sub181 { Sub181{e: Default::default(), f: F181::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type181 { pub sub: Sub181, pub e: E181, pub f: F181, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type181 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type181{sub: Default::default(), e: Default::default(), f: F181::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type181 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E182 { A, B, C }
impl Default for E182 { fn default() -> E182 { E182::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F182 { A, B, C }
impl Default for F182 { fn default() -> F182 { F182::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub182 { pub e: E182, pub f: F182, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub182 { pub fn new(name: String) -> Sub182 { Sub182{e: Default::default(), f: F182::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type182 { pub sub: Sub182, pub e: E182, pub f: F182, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type182 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type182{sub: Default::default(), e: Default::default(), f: F182::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type182 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E183 { A, B, C }
impl Default for E183 { fn default() -> E183 { E183::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F183 { A, B, C }
impl Default for F183 { fn default() -> F183 { F183::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub183 { pub e: E183, pub f: F183, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub183 { pub fn new(name: String) -> Sub183 { Sub183{e: Default::default(), f: F183::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type183 { pub sub: Sub183, pub e: E183, pub f: F183, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type183 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type183{sub: Default::default(), e: Default::default(), f: F183::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type183 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E184 { A, B, C }
impl Default for E184 { fn default() -> E184 { E184::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F184 { A, B, C }
impl Default for F184 { fn default() -> F184 { F184::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub184 { pub e: E184, pub f: F184, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub184 { pub fn new(name: String) -> Sub184 { Sub184{e: Default::default(), f: F184::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type184 { pub sub: Sub184, pub e: E184, pub f: F184, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type184 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type184{sub: Default::default(), e: Default::default(), f: F184::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type184 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E185 { A, B, C }
impl Default for E185 { fn default() -> E185 { E185::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F185 { A, B, C }
impl Default for F185 { fn default() -> F185 { F185::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub185 { pub e: E185, pub f: F185, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub185 { pub fn new(name: String) -> Sub185 { Sub185{e: Default::default(), f: F185::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type185 { pub sub: Sub185, pub e: E185, pub f: F185, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type185 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type185{sub: Default::default(), e: Default::default(), f: F185::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type185 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E186 { A, B, C }
impl Default for E186 { fn default() -> E186 { E186::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F186 { A, B, C }
impl Default for F186 { fn default() -> F186 { F186::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub186 { pub e: E186, pub f: F186, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub186 { pub fn new(name: String) -> Sub186 { Sub186{e: Default::default(), f: F186::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type186 { pub sub: Sub186, pub e: E186, pub f: F186, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type186 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type186{sub: Default::default(), e: Default::default(), f: F186::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type186 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E187 { A, B, C }
impl Default for E187 { fn default() -> E187 { E187::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F187 { A, B, C }
impl Default for F187 { fn default() -> F187 { F187::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub187 { pub e: E187, pub f: F187, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub187 { pub fn new(name: String) -> Sub187 { Sub187{e: Default::default(), f: F187::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type187 { pub sub: Sub187, pub e: E187, pub f: F187, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type187 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type187{sub: Default::default(), e: Default::default(), f: F187::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type187 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E188 { A, B, C }
impl Default for E188 { fn default() -> E188 { E188::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F188 { A, B, C }
impl Default for F188 { fn default() -> F188 { F188::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub188 { pub e: E188, pub f: F188, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub188 { pub fn new(name: String) -> Sub188 { Sub188{e: Default::default(), f: F188::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type188 { pub sub: Sub188, pub e: E188, pub f: F188, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type188 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type188{sub: Default::default(), e: Default::default(), f: F188::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type188 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E189 { A, B, C }
impl Default for E189 { fn default() -> E189 { E189::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F189 { A, B, C }
impl Default for F189 { fn default() -> F189 { F189::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub189 { pub e: E189, pub f: F189, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub189 { pub fn new(name: String) -> Sub189 { Sub189{e: Default::default(), f: F189::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type189 { pub sub: Sub189, pub e: E189, pub f: F189, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type189 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type189{sub: Default::default(), e: Default::default(), f: F189::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type189 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E190 { A, B, C }
impl Default for E190 { fn default() -> E190 { E190::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F190 { A, B, C }
impl Default for F190 { fn default() -> F190 { F190::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub190 { pub e: E190, pub f: F190, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub190 { pub fn new(name: String) -> Sub190 { Sub190{e: Default::default(), f: F190::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type190 { pub sub: Sub190, pub e: E190, pub f: F190, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type190 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type190{sub: Default::default(), e: Default::default(), f: F190::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type190 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E191 { A, B, C }
impl Default for E191 { fn default() -> E191 { E191::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F191 { A, B, C }
impl Default for F191 { fn default() -> F191 { F191::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub191 { pub e: E191, pub f: F191, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub191 { pub fn new(name: String) -> Sub191 { Sub191{e: Default::default(), f: F191::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type191 { pub sub: Sub191, pub e: E191, pub f: F191, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type191 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type191{sub: Default::default(), e: Default::default(), f: F191::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type191 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E192 { A, B, C }
impl Default for E192 { fn default() -> E192 { E192::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F192 { A, B, C }
impl Default for F192 { fn default() -> F192 { F192::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub192 { pub e: E192, pub f: F192, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub192 { pub fn new(name: String) -> Sub192 { Sub192{e: Default::default(), f: F192::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type192 { pub sub: Sub192, pub e: E192, pub f: F192, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type192 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type192{sub: Default::default(), e: Default::default(), f: F192::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type192 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E193 { A, B, C }
impl Default for E193 { fn default() -> E193 { E193::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F193 { A, B, C }
impl Default for F193 { fn default() -> F193 { F193::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub193 { pub e: E193, pub f: F193, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub193 { pub fn new(name: String) -> Sub193 { Sub193{e: Default::default(), f: F193::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type193 { pub sub: Sub193, pub e: E193, pub f: F193, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type193 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type193{sub: Default::default(), e: Default::default(), f: F193::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type193 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E194 { A, B, C }
impl Default for E194 { fn default() -> E194 { E194::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F194 { A, B, C }
impl Default for F194 { fn default() -> F194 { F194::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub194 { pub e: E194, pub f: F194, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub194 { pub fn new(name: String) -> Sub194 { Sub194{e: Default::default(), f: F194::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type194 { pub sub: Sub194, pub e: E194, pub f: F194, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type194 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type194{sub: Default::default(), e: Default::default(), f: F194::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type194 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E195 { A, B, C }
impl Default for E195 { fn default() -> E195 { E195::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F195 { A, B, C }
impl Default for F195 { fn default() -> F195 { F195::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub195 { pub e: E195, pub f: F195, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub195 { pub fn new(name: String) -> Sub195 { Sub195{e: Default::default(), f: F195::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type195 { pub sub: Sub195, pub e: E195, pub f: F195, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type195 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type195{sub: Default::default(), e: Default::default(), f: F195::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type195 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E196 { A, B, C }
impl Default for E196 { fn default() -> E196 { E196::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F196 { A, B, C }
impl Default for F196 { fn default() -> F196 { F196::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub196 { pub e: E196, pub f: F196, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub196 { pub fn new(name: String) -> Sub196 { Sub196{e: Default::default(), f: F196::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type196 { pub sub: Sub196, pub e: E196, pub f: F196, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type196 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type196{sub: Default::default(), e: Default::default(), f: F196::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type196 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E197 { A, B, C }
impl Default for E197 { fn default() -> E197 { E197::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F197 { A, B, C }
impl Default for F197 { fn default() -> F197 { F197::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub197 { pub e: E197, pub f: F197, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub197 { pub fn new(name: String) -> Sub197 { Sub197{e: Default::default(), f: F197::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type197 { pub sub: Sub197, pub e: E197, pub f: F197, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type197 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type197{sub: Default::default(), e: Default::default(), f: F197::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type197 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E198 { A, B, C }
impl Default for E198 { fn default() -> E198 { E198::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F198 { A, B, C }
impl Default for F198 { fn default() -> F198 { F198::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub198 { pub e: E198, pub f: F198, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub198 { pub fn new(name: String) -> Sub198 { Sub198{e: Default::default(), f: F198::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type198 { pub sub: Sub198, pub e: E198, pub f: F198, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type198 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type198{sub: Default::default(), e: Default::default(), f: F198::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type198 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E199 { A, B, C }
impl Default for E199 { fn default() -> E199 { E199::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F199 { A, B, C }
impl Default for F199 { fn default() -> F199 { F199::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub199 { pub e: E199, pub f: F199, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub199 { pub fn new(name: String) -> Sub199 { Sub199{e: Default::default(), f: F199::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type199 { pub sub: Sub199, pub e: E199, pub f: F199, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type199 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type199{sub: Default::default(), e: Default::default(), f: F199::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type199 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E200 { A, B, C }
impl Default for E200 { fn default() -> E200 { E200::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F200 { A, B, C }
impl Default for F200 { fn default() -> F200 { F200::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub200 { pub e: E200, pub f: F200, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub200 { pub fn new(name: String) -> Sub200 { Sub200{e: Default::default(), f: F200::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type200 { pub sub: Sub200, pub e: E200, pub f: F200, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type200 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type200{sub: Default::default(), e: Default::default(), f: F200::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type200 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E201 { A, B, C }
impl Default for E201 { fn default() -> E201 { E201::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F201 { A, B, C }
impl Default for F201 { fn default() -> F201 { F201::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub201 { pub e: E201, pub f: F201, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub201 { pub fn new(name: String) -> Sub201 { Sub201{e: Default::default(), f: F201::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type201 { pub sub: Sub201, pub e: E201, pub f: F201, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type201 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type201{sub: Default::default(), e: Default::default(), f: F201::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type201 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E202 { A, B, C }
impl Default for E202 { fn default() -> E202 { E202::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F202 { A, B, C }
impl Default for F202 { fn default() -> F202 { F202::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub202 { pub e: E202, pub f: F202, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub202 { pub fn new(name: String) -> Sub202 { Sub202{e: Default::default(), f: F202::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type202 { pub sub: Sub202, pub e: E202, pub f: F202, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type202 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type202{sub: Default::default(), e: Default::default(), f: F202::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type202 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E203 { A, B, C }
impl Default for E203 { fn default() -> E203 { E203::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F203 { A, B, C }
impl Default for F203 { fn default() -> F203 { F203::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub203 { pub e: E203, pub f: F203, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub203 { pub fn new(name: String) -> Sub203 { Sub203{e: Default::default(), f: F203::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type203 { pub sub: Sub203, pub e: E203, pub f: F203, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type203 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type203{sub: Default::default(), e: Default::default(), f: F203::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type203 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E204 { A, B, C }
impl Default for E204 { fn default() -> E204 { E204::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F204 { A, B, C }
impl Default for F204 { fn default() -> F204 { F204::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub204 { pub e: E204, pub f: F204, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub204 { pub fn new(name: String) -> Sub204 { Sub204{e: Default::default(), f: F204::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type204 { pub sub: Sub204, pub e: E204, pub f: F204, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type204 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type204{sub: Default::default(), e: Default::default(), f: F204::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type204 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E205 { A, B, C }
impl Default for E205 { fn default() -> E205 { E205::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F205 { A, B, C }
impl Default for F205 { fn default() -> F205 { F205::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub205 { pub e: E205, pub f: F205, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub205 { pub fn new(name: String) -> Sub205 { Sub205{e: Default::default(), f: F205::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type205 { pub sub: Sub205, pub e: E205, pub f: F205, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type205 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type205{sub: Default::default(), e: Default::default(), f: F205::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type205 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E206 { A, B, C }
impl Default for E206 { fn default() -> E206 { E206::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F206 { A, B, C }
impl Default for F206 { fn default() -> F206 { F206::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub206 { pub e: E206, pub f: F206, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub206 { pub fn new(name: String) -> Sub206 { Sub206{e: Default::default(), f: F206::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type206 { pub sub: Sub206, pub e: E206, pub f: F206, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type206 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type206{sub: Default::default(), e: Default::default(), f: F206::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type206 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E207 { A, B, C }
impl Default for E207 { fn default() -> E207 { E207::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F207 { A, B, C }
impl Default for F207 { fn default() -> F207 { F207::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub207 { pub e: E207, pub f: F207, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub207 { pub fn new(name: String) -> Sub207 { Sub207{e: Default::default(), f: F207::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type207 { pub sub: Sub207, pub e: E207, pub f: F207, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type207 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type207{sub: Default::default(), e: Default::default(), f: F207::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type207 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E208 { A, B, C }
impl Default for E208 { fn default() -> E208 { E208::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F208 { A, B, C }
impl Default for F208 { fn default() -> F208 { F208::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub208 { pub e: E208, pub f: F208, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub208 { pub fn new(name: String) -> Sub208 { Sub208{e: Default::default(), f: F208::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type208 { pub sub: Sub208, pub e: E208, pub f: F208, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type208 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type208{sub: Default::default(), e: Default::default(), f: F208::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type208 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E209 { A, B, C }
impl Default for E209 { fn default() -> E209 { E209::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F209 { A, B, C }
impl Default for F209 { fn default() -> F209 { F209::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub209 { pub e: E209, pub f: F209, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub209 { pub fn new(name: String) -> Sub209 { Sub209{e: Default::default(), f: F209::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type209 { pub sub: Sub209, pub e: E209, pub f: F209, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type209 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type209{sub: Default::default(), e: Default::default(), f: F209::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type209 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E210 { A, B, C }
impl Default for E210 { fn default() -> E210 { E210::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F210 { A, B, C }
impl Default for F210 { fn default() -> F210 { F210::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub210 { pub e: E210, pub f: F210, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub210 { pub fn new(name: String) -> Sub210 { Sub210{e: Default::default(), f: F210::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type210 { pub sub: Sub210, pub e: E210, pub f: F210, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type210 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type210{sub: Default::default(), e: Default::default(), f: F210::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type210 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E211 { A, B, C }
impl Default for E211 { fn default() -> E211 { E211::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F211 { A, B, C }
impl Default for F211 { fn default() -> F211 { F211::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub211 { pub e: E211, pub f: F211, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub211 { pub fn new(name: String) -> Sub211 { Sub211{e: Default::default(), f: F211::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type211 { pub sub: Sub211, pub e: E211, pub f: F211, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type211 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type211{sub: Default::default(), e: Default::default(), f: F211::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type211 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E212 { A, B, C }
impl Default for E212 { fn default() -> E212 { E212::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F212 { A, B, C }
impl Default for F212 { fn default() -> F212 { F212::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub212 { pub e: E212, pub f: F212, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub212 { pub fn new(name: String) -> Sub212 { Sub212{e: Default::default(), f: F212::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type212 { pub sub: Sub212, pub e: E212, pub f: F212, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type212 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type212{sub: Default::default(), e: Default::default(), f: F212::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type212 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E213 { A, B, C }
impl Default for E213 { fn default() -> E213 { E213::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F213 { A, B, C }
impl Default for F213 { fn default() -> F213 { F213::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub213 { pub e: E213, pub f: F213, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub213 { pub fn new(name: String) -> Sub213 { Sub213{e: Default::default(), f: F213::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type213 { pub sub: Sub213, pub e: E213, pub f: F213, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type213 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type213{sub: Default::default(), e: Default::default(), f: F213::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type213 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E214 { A, B, C }
impl Default for E214 { fn default() -> E214 { E214::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F214 { A, B, C }
impl Default for F214 { fn default() -> F214 { F214::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub214 { pub e: E214, pub f: F214, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub214 { pub fn new(name: String) -> Sub214 { Sub214{e: Default::default(), f: F214::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type214 { pub sub: Sub214, pub e: E214, pub f: F214, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type214 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type214{sub: Default::default(), e: Default::default(), f: F214::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type214 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E215 { A, B, C }
impl Default for E215 { fn default() -> E215 { E215::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F215 { A, B, C }
impl Default for F215 { fn default() -> F215 { F215::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub215 { pub e: E215, pub f: F215, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub215 { pub fn new(name: String) -> Sub215 { Sub215{e: Default::default(), f: F215::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type215 { pub sub: Sub215, pub e: E215, pub f: F215, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type215 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type215{sub: Default::default(), e: Default::default(), f: F215::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type215 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E216 { A, B, C }
impl Default for E216 { fn default() -> E216 { E216::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F216 { A, B, C }
impl Default for F216 { fn default() -> F216 { F216::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub216 { pub e: E216, pub f: F216, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub216 { pub fn new(name: String) -> Sub216 { Sub216{e: Default::default(), f: F216::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type216 { pub sub: Sub216, pub e: E216, pub f: F216, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type216 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type216{sub: Default::default(), e: Default::default(), f: F216::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type216 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E217 { A, B, C }
impl Default for E217 { fn default() -> E217 { E217::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F217 { A, B, C }
impl Default for F217 { fn default() -> F217 { F217::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub217 { pub e: E217, pub f: F217, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub217 { pub fn new(name: String) -> Sub217 { Sub217{e: Default::default(), f: F217::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type217 { pub sub: Sub217, pub e: E217, pub f: F217, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type217 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type217{sub: Default::default(), e: Default::default(), f: F217::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type217 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E218 { A, B, C }
impl Default for E218 { fn default() -> E218 { E218::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F218 { A, B, C }
impl Default for F218 { fn default() -> F218 { F218::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub218 { pub e: E218, pub f: F218, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub218 { pub fn new(name: String) -> Sub218 { Sub218{e: Default::default(), f: F218::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type218 { pub sub: Sub218, pub e: E218, pub f: F218, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type218 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type218{sub: Default::default(), e: Default::default(), f: F218::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type218 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E219 { A, B, C }
impl Default for E219 { fn default() -> E219 { E219::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F219 { A, B, C }
impl Default for F219 { fn default() -> F219 { F219::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub219 { pub e: E219, pub f: F219, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub219 { pub fn new(name: String) -> Sub219 { Sub219{e: Default::default(), f: F219::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type219 { pub sub: Sub219, pub e: E219, pub f: F219, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type219 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type219{sub: Default::default(), e: Default::default(), f: F219::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type219 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E220 { A, B, C }
impl Default for E220 { fn default() -> E220 { E220::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F220 { A, B, C }
impl Default for F220 { fn default() -> F220 { F220::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub220 { pub e: E220, pub f: F220, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub220 { pub fn new(name: String) -> Sub220 { Sub220{e: Default::default(), f: F220::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type220 { pub sub: Sub220, pub e: E220, pub f: F220, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type220 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type220{sub: Default::default(), e: Default::default(), f: F220::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type220 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E221 { A, B, C }
impl Default for E221 { fn default() -> E221 { E221::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F221 { A, B, C }
impl Default for F221 { fn default() -> F221 { F221::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub221 { pub e: E221, pub f: F221, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub221 { pub fn new(name: String) -> Sub221 { Sub221{e: Default::default(), f: F221::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type221 { pub sub: Sub221, pub e: E221, pub f: F221, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type221 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type221{sub: Default::default(), e: Default::default(), f: F221::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type221 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E222 { A, B, C }
impl Default for E222 { fn default() -> E222 { E222::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F222 { A, B, C }
impl Default for F222 { fn default() -> F222 { F222::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub222 { pub e: E222, pub f: F222, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub222 { pub fn new(name: String) -> Sub222 { Sub222{e: Default::default(), f: F222::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type222 { pub sub: Sub222, pub e: E222, pub f: F222, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type222 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type222{sub: Default::default(), e: Default::default(), f: F222::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type222 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E223 { A, B, C }
impl Default for E223 { fn default() -> E223 { E223::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F223 { A, B, C }
impl Default for F223 { fn default() -> F223 { F223::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub223 { pub e: E223, pub f: F223, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub223 { pub fn new(name: String) -> Sub223 { Sub223{e: Default::default(), f: F223::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type223 { pub sub: Sub223, pub e: E223, pub f: F223, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type223 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type223{sub: Default::default(), e: Default::default(), f: F223::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type223 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E224 { A, B, C }
impl Default for E224 { fn default() -> E224 { E224::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F224 { A, B, C }
impl Default for F224 { fn default() -> F224 { F224::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub224 { pub e: E224, pub f: F224, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub224 { pub fn new(name: String) -> Sub224 { Sub224{e: Default::default(), f: F224::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type224 { pub sub: Sub224, pub e: E224, pub f: F224, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type224 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type224{sub: Default::default(), e: Default::default(), f: F224::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type224 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E225 { A, B, C }
impl Default for E225 { fn default() -> E225 { E225::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F225 { A, B, C }
impl Default for F225 { fn default() -> F225 { F225::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub225 { pub e: E225, pub f: F225, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub225 { pub fn new(name: String) -> Sub225 { Sub225{e: Default::default(), f: F225::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type225 { pub sub: Sub225, pub e: E225, pub f: F225, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type225 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type225{sub: Default::default(), e: Default::default(), f: F225::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type225 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E226 { A, B, C }
impl Default for E226 { fn default() -> E226 { E226::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F226 { A, B, C }
impl Default for F226 { fn default() -> F226 { F226::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub226 { pub e: E226, pub f: F226, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub226 { pub fn new(name: String) -> Sub226 { Sub226{e: Default::default(), f: F226::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type226 { pub sub: Sub226, pub e: E226, pub f: F226, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type226 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type226{sub: Default::default(), e: Default::default(), f: F226::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type226 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E227 { A, B, C }
impl Default for E227 { fn default() -> E227 { E227::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F227 { A, B, C }
impl Default for F227 { fn default() -> F227 { F227::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub227 { pub e: E227, pub f: F227, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub227 { pub fn new(name: String) -> Sub227 { Sub227{e: Default::default(), f: F227::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type227 { pub sub: Sub227, pub e: E227, pub f: F227, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type227 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type227{sub: Default::default(), e: Default::default(), f: F227::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type227 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E228 { A, B, C }
impl Default for E228 { fn default() -> E228 { E228::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F228 { A, B, C }
impl Default for F228 { fn default() -> F228 { F228::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub228 { pub e: E228, pub f: F228, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub228 { pub fn new(name: String) -> Sub228 { Sub228{e: Default::default(), f: F228::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type228 { pub sub: Sub228, pub e: E228, pub f: F228, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type228 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type228{sub: Default::default(), e: Default::default(), f: F228::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type228 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E229 { A, B, C }
impl Default for E229 { fn default() -> E229 { E229::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F229 { A, B, C }
impl Default for F229 { fn default() -> F229 { F229::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub229 { pub e: E229, pub f: F229, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub229 { pub fn new(name: String) -> Sub229 { Sub229{e: Default::default(), f: F229::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type229 { pub sub: Sub229, pub e: E229, pub f: F229, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type229 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type229{sub: Default::default(), e: Default::default(), f: F229::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type229 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E230 { A, B, C }
impl Default for E230 { fn default() -> E230 { E230::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F230 { A, B, C }
impl Default for F230 { fn default() -> F230 { F230::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub230 { pub e: E230, pub f: F230, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub230 { pub fn new(name: String) -> Sub230 { Sub230{e: Default::default(), f: F230::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type230 { pub sub: Sub230, pub e: E230, pub f: F230, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type230 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type230{sub: Default::default(), e: Default::default(), f: F230::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type230 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E231 { A, B, C }
impl Default for E231 { fn default() -> E231 { E231::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F231 { A, B, C }
impl Default for F231 { fn default() -> F231 { F231::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub231 { pub e: E231, pub f: F231, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub231 { pub fn new(name: String) -> Sub231 { Sub231{e: Default::default(), f: F231::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type231 { pub sub: Sub231, pub e: E231, pub f: F231, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type231 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type231{sub: Default::default(), e: Default::default(), f: F231::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type231 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E232 { A, B, C }
impl Default for E232 { fn default() -> E232 { E232::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F232 { A, B, C }
impl Default for F232 { fn default() -> F232 { F232::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub232 { pub e: E232, pub f: F232, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub232 { pub fn new(name: String) -> Sub232 { Sub232{e: Default::default(), f: F232::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type232 { pub sub: Sub232, pub e: E232, pub f: F232, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type232 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type232{sub: Default::default(), e: Default::default(), f: F232::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type232 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E233 { A, B, C }
impl Default for E233 { fn default() -> E233 { E233::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F233 { A, B, C }
impl Default for F233 { fn default() -> F233 { F233::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub233 { pub e: E233, pub f: F233, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub233 { pub fn new(name: String) -> Sub233 { Sub233{e: Default::default(), f: F233::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type233 { pub sub: Sub233, pub e: E233, pub f: F233, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type233 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type233{sub: Default::default(), e: Default::default(), f: F233::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type233 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E234 { A, B, C }
impl Default for E234 { fn default() -> E234 { E234::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F234 { A, B, C }
impl Default for F234 { fn default() -> F234 { F234::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub234 { pub e: E234, pub f: F234, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub234 { pub fn new(name: String) -> Sub234 { Sub234{e: Default::default(), f: F234::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type234 { pub sub: Sub234, pub e: E234, pub f: F234, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type234 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type234{sub: Default::default(), e: Default::default(), f: F234::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type234 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E235 { A, B, C }
impl Default for E235 { fn default() -> E235 { E235::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F235 { A, B, C }
impl Default for F235 { fn default() -> F235 { F235::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub235 { pub e: E235, pub f: F235, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub235 { pub fn new(name: String) -> Sub235 { Sub235{e: Default::default(), f: F235::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type235 { pub sub: Sub235, pub e: E235, pub f: F235, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type235 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type235{sub: Default::default(), e: Default::default(), f: F235::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type235 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E236 { A, B, C }
impl Default for E236 { fn default() -> E236 { E236::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F236 { A, B, C }
impl Default for F236 { fn default() -> F236 { F236::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub236 { pub e: E236, pub f: F236, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub236 { pub fn new(name: String) -> Sub236 { Sub236{e: Default::default(), f: F236::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type236 { pub sub: Sub236, pub e: E236, pub f: F236, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type236 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type236{sub: Default::default(), e: Default::default(), f: F236::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type236 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E237 { A, B, C }
impl Default for E237 { fn default() -> E237 { E237::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F237 { A, B, C }
impl Default for F237 { fn default() -> F237 { F237::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub237 { pub e: E237, pub f: F237, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub237 { pub fn new(name: String) -> Sub237 { Sub237{e: Default::default(), f: F237::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type237 { pub sub: Sub237, pub e: E237, pub f: F237, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type237 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type237{sub: Default::default(), e: Default::default(), f: F237::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type237 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E238 { A, B, C }
impl Default for E238 { fn default() -> E238 { E238::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F238 { A, B, C }
impl Default for F238 { fn default() -> F238 { F238::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub238 { pub e: E238, pub f: F238, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub238 { pub fn new(name: String) -> Sub238 { Sub238{e: Default::default(), f: F238::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type238 { pub sub: Sub238, pub e: E238, pub f: F238, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type238 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type238{sub: Default::default(), e: Default::default(), f: F238::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type238 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E239 { A, B, C }
impl Default for E239 { fn default() -> E239 { E239::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F239 { A, B, C }
impl Default for F239 { fn default() -> F239 { F239::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub239 { pub e: E239, pub f: F239, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub239 { pub fn new(name: String) -> Sub239 { Sub239{e: Default::default(), f: F239::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type239 { pub sub: Sub239, pub e: E239, pub f: F239, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type239 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type239{sub: Default::default(), e: Default::default(), f: F239::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type239 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E240 { A, B, C }
impl Default for E240 { fn default() -> E240 { E240::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F240 { A, B, C }
impl Default for F240 { fn default() -> F240 { F240::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub240 { pub e: E240, pub f: F240, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub240 { pub fn new(name: String) -> Sub240 { Sub240{e: Default::default(), f: F240::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type240 { pub sub: Sub240, pub e: E240, pub f: F240, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type240 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type240{sub: Default::default(), e: Default::default(), f: F240::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type240 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E241 { A, B, C }
impl Default for E241 { fn default() -> E241 { E241::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F241 { A, B, C }
impl Default for F241 { fn default() -> F241 { F241::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub241 { pub e: E241, pub f: F241, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub241 { pub fn new(name: String) -> Sub241 { Sub241{e: Default::default(), f: F241::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type241 { pub sub: Sub241, pub e: E241, pub f: F241, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type241 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type241{sub: Default::default(), e: Default::default(), f: F241::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type241 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E242 { A, B, C }
impl Default for E242 { fn default() -> E242 { E242::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F242 { A, B, C }
impl Default for F242 { fn default() -> F242 { F242::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub242 { pub e: E242, pub f: F242, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub242 { pub fn new(name: String) -> Sub242 { Sub242{e: Default::default(), f: F242::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type242 { pub sub: Sub242, pub e: E242, pub f: F242, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type242 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type242{sub: Default::default(), e: Default::default(), f: F242::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type242 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E243 { A, B, C }
impl Default for E243 { fn default() -> E243 { E243::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F243 { A, B, C }
impl Default for F243 { fn default() -> F243 { F243::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub243 { pub e: E243, pub f: F243, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub243 { pub fn new(name: String) -> Sub243 { Sub243{e: Default::default(), f: F243::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type243 { pub sub: Sub243, pub e: E243, pub f: F243, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type243 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type243{sub: Default::default(), e: Default::default(), f: F243::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type243 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E244 { A, B, C }
impl Default for E244 { fn default() -> E244 { E244::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F244 { A, B, C }
impl Default for F244 { fn default() -> F244 { F244::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub244 { pub e: E244, pub f: F244, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub244 { pub fn new(name: String) -> Sub244 { Sub244{e: Default::default(), f: F244::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type244 { pub sub: Sub244, pub e: E244, pub f: F244, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type244 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type244{sub: Default::default(), e: Default::default(), f: F244::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type244 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E245 { A, B, C }
impl Default for E245 { fn default() -> E245 { E245::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F245 { A, B, C }
impl Default for F245 { fn default() -> F245 { F245::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub245 { pub e: E245, pub f: F245, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub245 { pub fn new(name: String) -> Sub245 { Sub245{e: Default::default(), f: F245::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type245 { pub sub: Sub245, pub e: E245, pub f: F245, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type245 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type245{sub: Default::default(), e: Default::default(), f: F245::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type245 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E246 { A, B, C }
impl Default for E246 { fn default() -> E246 { E246::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F246 { A, B, C }
impl Default for F246 { fn default() -> F246 { F246::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub246 { pub e: E246, pub f: F246, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub246 { pub fn new(name: String) -> Sub246 { Sub246{e: Default::default(), f: F246::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type246 { pub sub: Sub246, pub e: E246, pub f: F246, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type246 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type246{sub: Default::default(), e: Default::default(), f: F246::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type246 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E247 { A, B, C }
impl Default for E247 { fn default() -> E247 { E247::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F247 { A, B, C }
impl Default for F247 { fn default() -> F247 { F247::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub247 { pub e: E247, pub f: F247, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub247 { pub fn new(name: String) -> Sub247 { Sub247{e: Default::default(), f: F247::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type247 { pub sub: Sub247, pub e: E247, pub f: F247, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type247 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type247{sub: Default::default(), e: Default::default(), f: F247::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type247 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E248 { A, B, C }
impl Default for E248 { fn default() -> E248 { E248::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F248 { A, B, C }
impl Default for F248 { fn default() -> F248 { F248::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub248 { pub e: E248, pub f: F248, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub248 { pub fn new(name: String) -> Sub248 { Sub248{e: Default::default(), f: F248::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type248 { pub sub: Sub248, pub e: E248, pub f: F248, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type248 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type248{sub: Default::default(), e: Default::default(), f: F248::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type248 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E249 { A, B, C }
impl Default for E249 { fn default() -> E249 { E249::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F249 { A, B, C }
impl Default for F249 { fn default() -> F249 { F249::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub249 { pub e: E249, pub f: F249, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub249 { pub fn new(name: String) -> Sub249 { Sub249{e: Default::default(), f: F249::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type249 { pub sub: Sub249, pub e: E249, pub f: F249, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type249 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type249{sub: Default::default(), e: Default::default(), f: F249::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type249 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E250 { A, B, C }
impl Default for E250 { fn default() -> E250 { E250::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F250 { A, B, C }
impl Default for F250 { fn default() -> F250 { F250::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub250 { pub e: E250, pub f: F250, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub250 { pub fn new(name: String) -> Sub250 { Sub250{e: Default::default(), f: F250::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type250 { pub sub: Sub250, pub e: E250, pub f: F250, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type250 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type250{sub: Default::default(), e: Default::default(), f: F250::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type250 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E251 { A, B, C }
impl Default for E251 { fn default() -> E251 { E251::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F251 { A, B, C }
impl Default for F251 { fn default() -> F251 { F251::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub251 { pub e: E251, pub f: F251, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub251 { pub fn new(name: String) -> Sub251 { Sub251{e: Default::default(), f: F251::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type251 { pub sub: Sub251, pub e: E251, pub f: F251, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type251 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type251{sub: Default::default(), e: Default::default(), f: F251::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type251 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E252 { A, B, C }
impl Default for E252 { fn default() -> E252 { E252::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F252 { A, B, C }
impl Default for F252 { fn default() -> F252 { F252::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub252 { pub e: E252, pub f: F252, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub252 { pub fn new(name: String) -> Sub252 { Sub252{e: Default::default(), f: F252::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type252 { pub sub: Sub252, pub e: E252, pub f: F252, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type252 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type252{sub: Default::default(), e: Default::default(), f: F252::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type252 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E253 { A, B, C }
impl Default for E253 { fn default() -> E253 { E253::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F253 { A, B, C }
impl Default for F253 { fn default() -> F253 { F253::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub253 { pub e: E253, pub f: F253, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub253 { pub fn new(name: String) -> Sub253 { Sub253{e: Default::default(), f: F253::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type253 { pub sub: Sub253, pub e: E253, pub f: F253, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type253 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type253{sub: Default::default(), e: Default::default(), f: F253::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type253 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E254 { A, B, C }
impl Default for E254 { fn default() -> E254 { E254::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F254 { A, B, C }
impl Default for F254 { fn default() -> F254 { F254::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub254 { pub e: E254, pub f: F254, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub254 { pub fn new(name: String) -> Sub254 { Sub254{e: Default::default(), f: F254::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type254 { pub sub: Sub254, pub e: E254, pub f: F254, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type254 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type254{sub: Default::default(), e: Default::default(), f: F254::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type254 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E255 { A, B, C }
impl Default for E255 { fn default() -> E255 { E255::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F255 { A, B, C }
impl Default for F255 { fn default() -> F255 { F255::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub255 { pub e: E255, pub f: F255, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub255 { pub fn new(name: String) -> Sub255 { Sub255{e: Default::default(), f: F255::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type255 { pub sub: Sub255, pub e: E255, pub f: F255, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type255 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type255{sub: Default::default(), e: Default::default(), f: F255::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type255 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E256 { A, B, C }
impl Default for E256 { fn default() -> E256 { E256::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F256 { A, B, C }
impl Default for F256 { fn default() -> F256 { F256::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub256 { pub e: E256, pub f: F256, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub256 { pub fn new(name: String) -> Sub256 { Sub256{e: Default::default(), f: F256::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type256 { pub sub: Sub256, pub e: E256, pub f: F256, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type256 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type256{sub: Default::default(), e: Default::default(), f: F256::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type256 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E257 { A, B, C }
impl Default for E257 { fn default() -> E257 { E257::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F257 { A, B, C }
impl Default for F257 { fn default() -> F257 { F257::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub257 { pub e: E257, pub f: F257, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub257 { pub fn new(name: String) -> Sub257 { Sub257{e: Default::default(), f: F257::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type257 { pub sub: Sub257, pub e: E257, pub f: F257, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type257 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type257{sub: Default::default(), e: Default::default(), f: F257::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type257 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E258 { A, B, C }
impl Default for E258 { fn default() -> E258 { E258::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F258 { A, B, C }
impl Default for F258 { fn default() -> F258 { F258::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub258 { pub e: E258, pub f: F258, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub258 { pub fn new(name: String) -> Sub258 { Sub258{e: Default::default(), f: F258::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type258 { pub sub: Sub258, pub e: E258, pub f: F258, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type258 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type258{sub: Default::default(), e: Default::default(), f: F258::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type258 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E259 { A, B, C }
impl Default for E259 { fn default() -> E259 { E259::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F259 { A, B, C }
impl Default for F259 { fn default() -> F259 { F259::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub259 { pub e: E259, pub f: F259, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub259 { pub fn new(name: String) -> Sub259 { Sub259{e: Default::default(), f: F259::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type259 { pub sub: Sub259, pub e: E259, pub f: F259, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type259 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type259{sub: Default::default(), e: Default::default(), f: F259::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type259 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E260 { A, B, C }
impl Default for E260 { fn default() -> E260 { E260::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F260 { A, B, C }
impl Default for F260 { fn default() -> F260 { F260::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub260 { pub e: E260, pub f: F260, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub260 { pub fn new(name: String) -> Sub260 { Sub260{e: Default::default(), f: F260::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type260 { pub sub: Sub260, pub e: E260, pub f: F260, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type260 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type260{sub: Default::default(), e: Default::default(), f: F260::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type260 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E261 { A, B, C }
impl Default for E261 { fn default() -> E261 { E261::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F261 { A, B, C }
impl Default for F261 { fn default() -> F261 { F261::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub261 { pub e: E261, pub f: F261, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub261 { pub fn new(name: String) -> Sub261 { Sub261{e: Default::default(), f: F261::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type261 { pub sub: Sub261, pub e: E261, pub f: F261, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type261 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type261{sub: Default::default(), e: Default::default(), f: F261::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type261 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E262 { A, B, C }
impl Default for E262 { fn default() -> E262 { E262::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F262 { A, B, C }
impl Default for F262 { fn default() -> F262 { F262::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub262 { pub e: E262, pub f: F262, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub262 { pub fn new(name: String) -> Sub262 { Sub262{e: Default::default(), f: F262::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type262 { pub sub: Sub262, pub e: E262, pub f: F262, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type262 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type262{sub: Default::default(), e: Default::default(), f: F262::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type262 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E263 { A, B, C }
impl Default for E263 { fn default() -> E263 { E263::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F263 { A, B, C }
impl Default for F263 { fn default() -> F263 { F263::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub263 { pub e: E263, pub f: F263, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub263 { pub fn new(name: String) -> Sub263 { Sub263{e: Default::default(), f: F263::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type263 { pub sub: Sub263, pub e: E263, pub f: F263, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type263 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type263{sub: Default::default(), e: Default::default(), f: F263::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type263 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E264 { A, B, C }
impl Default for E264 { fn default() -> E264 { E264::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F264 { A, B, C }
impl Default for F264 { fn default() -> F264 { F264::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub264 { pub e: E264, pub f: F264, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub264 { pub fn new(name: String) -> Sub264 { Sub264{e: Default::default(), f: F264::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type264 { pub sub: Sub264, pub e: E264, pub f: F264, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type264 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type264{sub: Default::default(), e: Default::default(), f: F264::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type264 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E265 { A, B, C }
impl Default for E265 { fn default() -> E265 { E265::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F265 { A, B, C }
impl Default for F265 { fn default() -> F265 { F265::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub265 { pub e: E265, pub f: F265, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub265 { pub fn new(name: String) -> Sub265 { Sub265{e: Default::default(), f: F265::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type265 { pub sub: Sub265, pub e: E265, pub f: F265, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type265 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type265{sub: Default::default(), e: Default::default(), f: F265::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type265 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E266 { A, B, C }
impl Default for E266 { fn default() -> E266 { E266::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F266 { A, B, C }
impl Default for F266 { fn default() -> F266 { F266::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub266 { pub e: E266, pub f: F266, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub266 { pub fn new(name: String) -> Sub266 { Sub266{e: Default::default(), f: F266::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type266 { pub sub: Sub266, pub e: E266, pub f: F266, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type266 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type266{sub: Default::default(), e: Default::default(), f: F266::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type266 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E267 { A, B, C }
impl Default for E267 { fn default() -> E267 { E267::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F267 { A, B, C }
impl Default for F267 { fn default() -> F267 { F267::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub267 { pub e: E267, pub f: F267, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub267 { pub fn new(name: String) -> Sub267 { Sub267{e: Default::default(), f: F267::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type267 { pub sub: Sub267, pub e: E267, pub f: F267, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type267 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type267{sub: Default::default(), e: Default::default(), f: F267::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type267 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E268 { A, B, C }
impl Default for E268 { fn default() -> E268 { E268::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F268 { A, B, C }
impl Default for F268 { fn default() -> F268 { F268::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub268 { pub e: E268, pub f: F268, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub268 { pub fn new(name: String) -> Sub268 { Sub268{e: Default::default(), f: F268::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type268 { pub sub: Sub268, pub e: E268, pub f: F268, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type268 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type268{sub: Default::default(), e: Default::default(), f: F268::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type268 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E269 { A, B, C }
impl Default for E269 { fn default() -> E269 { E269::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F269 { A, B, C }
impl Default for F269 { fn default() -> F269 { F269::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub269 { pub e: E269, pub f: F269, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub269 { pub fn new(name: String) -> Sub269 { Sub269{e: Default::default(), f: F269::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type269 { pub sub: Sub269, pub e: E269, pub f: F269, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type269 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type269{sub: Default::default(), e: Default::default(), f: F269::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type269 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E270 { A, B, C }
impl Default for E270 { fn default() -> E270 { E270::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F270 { A, B, C }
impl Default for F270 { fn default() -> F270 { F270::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub270 { pub e: E270, pub f: F270, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub270 { pub fn new(name: String) -> Sub270 { Sub270{e: Default::default(), f: F270::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type270 { pub sub: Sub270, pub e: E270, pub f: F270, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type270 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type270{sub: Default::default(), e: Default::default(), f: F270::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type270 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E271 { A, B, C }
impl Default for E271 { fn default() -> E271 { E271::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F271 { A, B, C }
impl Default for F271 { fn default() -> F271 { F271::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub271 { pub e: E271, pub f: F271, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub271 { pub fn new(name: String) -> Sub271 { Sub271{e: Default::default(), f: F271::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type271 { pub sub: Sub271, pub e: E271, pub f: F271, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type271 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type271{sub: Default::default(), e: Default::default(), f: F271::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type271 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E272 { A, B, C }
impl Default for E272 { fn default() -> E272 { E272::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F272 { A, B, C }
impl Default for F272 { fn default() -> F272 { F272::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub272 { pub e: E272, pub f: F272, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub272 { pub fn new(name: String) -> Sub272 { Sub272{e: Default::default(), f: F272::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type272 { pub sub: Sub272, pub e: E272, pub f: F272, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type272 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type272{sub: Default::default(), e: Default::default(), f: F272::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type272 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E273 { A, B, C }
impl Default for E273 { fn default() -> E273 { E273::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F273 { A, B, C }
impl Default for F273 { fn default() -> F273 { F273::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub273 { pub e: E273, pub f: F273, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub273 { pub fn new(name: String) -> Sub273 { Sub273{e: Default::default(), f: F273::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type273 { pub sub: Sub273, pub e: E273, pub f: F273, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type273 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type273{sub: Default::default(), e: Default::default(), f: F273::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type273 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E274 { A, B, C }
impl Default for E274 { fn default() -> E274 { E274::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F274 { A, B, C }
impl Default for F274 { fn default() -> F274 { F274::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub274 { pub e: E274, pub f: F274, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub274 { pub fn new(name: String) -> Sub274 { Sub274{e: Default::default(), f: F274::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type274 { pub sub: Sub274, pub e: E274, pub f: F274, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type274 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type274{sub: Default::default(), e: Default::default(), f: F274::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type274 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E275 { A, B, C }
impl Default for E275 { fn default() -> E275 { E275::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F275 { A, B, C }
impl Default for F275 { fn default() -> F275 { F275::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub275 { pub e: E275, pub f: F275, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub275 { pub fn new(name: String) -> Sub275 { Sub275{e: Default::default(), f: F275::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type275 { pub sub: Sub275, pub e: E275, pub f: F275, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type275 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type275{sub: Default::default(), e: Default::default(), f: F275::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type275 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E276 { A, B, C }
impl Default for E276 { fn default() -> E276 { E276::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F276 { A, B, C }
impl Default for F276 { fn default() -> F276 { F276::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub276 { pub e: E276, pub f: F276, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub276 { pub fn new(name: String) -> Sub276 { Sub276{e: Default::default(), f: F276::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type276 { pub sub: Sub276, pub e: E276, pub f: F276, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type276 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type276{sub: Default::default(), e: Default::default(), f: F276::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type276 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E277 { A, B, C }
impl Default for E277 { fn default() -> E277 { E277::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F277 { A, B, C }
impl Default for F277 { fn default() -> F277 { F277::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub277 { pub e: E277, pub f: F277, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub277 { pub fn new(name: String) -> Sub277 { Sub277{e: Default::default(), f: F277::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type277 { pub sub: Sub277, pub e: E277, pub f: F277, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type277 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type277{sub: Default::default(), e: Default::default(), f: F277::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type277 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E278 { A, B, C }
impl Default for E278 { fn default() -> E278 { E278::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F278 { A, B, C }
impl Default for F278 { fn default() -> F278 { F278::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub278 { pub e: E278, pub f: F278, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub278 { pub fn new(name: String) -> Sub278 { Sub278{e: Default::default(), f: F278::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type278 { pub sub: Sub278, pub e: E278, pub f: F278, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type278 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type278{sub: Default::default(), e: Default::default(), f: F278::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type278 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E279 { A, B, C }
impl Default for E279 { fn default() -> E279 { E279::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F279 { A, B, C }
impl Default for F279 { fn default() -> F279 { F279::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub279 { pub e: E279, pub f: F279, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub279 { pub fn new(name: String) -> Sub279 { Sub279{e: Default::default(), f: F279::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type279 { pub sub: Sub279, pub e: E279, pub f: F279, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type279 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type279{sub: Default::default(), e: Default::default(), f: F279::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type279 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E280 { A, B, C }
impl Default for E280 { fn default() -> E280 { E280::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F280 { A, B, C }
impl Default for F280 { fn default() -> F280 { F280::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub280 { pub e: E280, pub f: F280, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub280 { pub fn new(name: String) -> Sub280 { Sub280{e: Default::default(), f: F280::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type280 { pub sub: Sub280, pub e: E280, pub f: F280, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type280 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type280{sub: Default::default(), e: Default::default(), f: F280::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type280 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E281 { A, B, C }
impl Default for E281 { fn default() -> E281 { E281::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F281 { A, B, C }
impl Default for F281 { fn default() -> F281 { F281::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub281 { pub e: E281, pub f: F281, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub281 { pub fn new(name: String) -> Sub281 { Sub281{e: Default::default(), f: F281::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type281 { pub sub: Sub281, pub e: E281, pub f: F281, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type281 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type281{sub: Default::default(), e: Default::default(), f: F281::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type281 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E282 { A, B, C }
impl Default for E282 { fn default() -> E282 { E282::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F282 { A, B, C }
impl Default for F282 { fn default() -> F282 { F282::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub282 { pub e: E282, pub f: F282, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub282 { pub fn new(name: String) -> Sub282 { Sub282{e: Default::default(), f: F282::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type282 { pub sub: Sub282, pub e: E282, pub f: F282, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type282 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type282{sub: Default::default(), e: Default::default(), f: F282::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type282 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E283 { A, B, C }
impl Default for E283 { fn default() -> E283 { E283::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F283 { A, B, C }
impl Default for F283 { fn default() -> F283 { F283::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub283 { pub e: E283, pub f: F283, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub283 { pub fn new(name: String) -> Sub283 { Sub283{e: Default::default(), f: F283::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type283 { pub sub: Sub283, pub e: E283, pub f: F283, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type283 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type283{sub: Default::default(), e: Default::default(), f: F283::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type283 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E284 { A, B, C }
impl Default for E284 { fn default() -> E284 { E284::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F284 { A, B, C }
impl Default for F284 { fn default() -> F284 { F284::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub284 { pub e: E284, pub f: F284, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub284 { pub fn new(name: String) -> Sub284 { Sub284{e: Default::default(), f: F284::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type284 { pub sub: Sub284, pub e: E284, pub f: F284, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type284 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type284{sub: Default::default(), e: Default::default(), f: F284::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type284 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E285 { A, B, C }
impl Default for E285 { fn default() -> E285 { E285::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F285 { A, B, C }
impl Default for F285 { fn default() -> F285 { F285::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub285 { pub e: E285, pub f: F285, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub285 { pub fn new(name: String) -> Sub285 { Sub285{e: Default::default(), f: F285::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type285 { pub sub: Sub285, pub e: E285, pub f: F285, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type285 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type285{sub: Default::default(), e: Default::default(), f: F285::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type285 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E286 { A, B, C }
impl Default for E286 { fn default() -> E286 { E286::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F286 { A, B, C }
impl Default for F286 { fn default() -> F286 { F286::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub286 { pub e: E286, pub f: F286, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub286 { pub fn new(name: String) -> Sub286 { Sub286{e: Default::default(), f: F286::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type286 { pub sub: Sub286, pub e: E286, pub f: F286, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type286 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type286{sub: Default::default(), e: Default::default(), f: F286::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type286 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E287 { A, B, C }
impl Default for E287 { fn default() -> E287 { E287::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F287 { A, B, C }
impl Default for F287 { fn default() -> F287 { F287::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub287 { pub e: E287, pub f: F287, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub287 { pub fn new(name: String) -> Sub287 { Sub287{e: Default::default(), f: F287::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type287 { pub sub: Sub287, pub e: E287, pub f: F287, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type287 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type287{sub: Default::default(), e: Default::default(), f: F287::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type287 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E288 { A, B, C }
impl Default for E288 { fn default() -> E288 { E288::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F288 { A, B, C }
impl Default for F288 { fn default() -> F288 { F288::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub288 { pub e: E288, pub f: F288, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub288 { pub fn new(name: String) -> Sub288 { Sub288{e: Default::default(), f: F288::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type288 { pub sub: Sub288, pub e: E288, pub f: F288, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type288 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type288{sub: Default::default(), e: Default::default(), f: F288::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type288 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E289 { A, B, C }
impl Default for E289 { fn default() -> E289 { E289::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F289 { A, B, C }
impl Default for F289 { fn default() -> F289 { F289::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub289 { pub e: E289, pub f: F289, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub289 { pub fn new(name: String) -> Sub289 { Sub289{e: Default::default(), f: F289::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type289 { pub sub: Sub289, pub e: E289, pub f: F289, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type289 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type289{sub: Default::default(), e: Default::default(), f: F289::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type289 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E290 { A, B, C }
impl Default for E290 { fn default() -> E290 { E290::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F290 { A, B, C }
impl Default for F290 { fn default() -> F290 { F290::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub290 { pub e: E290, pub f: F290, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub290 { pub fn new(name: String) -> Sub290 { Sub290{e: Default::default(), f: F290::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type290 { pub sub: Sub290, pub e: E290, pub f: F290, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type290 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type290{sub: Default::default(), e: Default::default(), f: F290::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type290 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E291 { A, B, C }
impl Default for E291 { fn default() -> E291 { E291::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F291 { A, B, C }
impl Default for F291 { fn default() -> F291 { F291::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub291 { pub e: E291, pub f: F291, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub291 { pub fn new(name: String) -> Sub291 { Sub291{e: Default::default(), f: F291::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type291 { pub sub: Sub291, pub e: E291, pub f: F291, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type291 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type291{sub: Default::default(), e: Default::default(), f: F291::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type291 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E292 { A, B, C }
impl Default for E292 { fn default() -> E292 { E292::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F292 { A, B, C }
impl Default for F292 { fn default() -> F292 { F292::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub292 { pub e: E292, pub f: F292, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub292 { pub fn new(name: String) -> Sub292 { Sub292{e: Default::default(), f: F292::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type292 { pub sub: Sub292, pub e: E292, pub f: F292, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type292 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type292{sub: Default::default(), e: Default::default(), f: F292::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type292 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E293 { A, B, C }
impl Default for E293 { fn default() -> E293 { E293::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F293 { A, B, C }
impl Default for F293 { fn default() -> F293 { F293::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub293 { pub e: E293, pub f: F293, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub293 { pub fn new(name: String) -> Sub293 { Sub293{e: Default::default(), f: F293::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type293 { pub sub: Sub293, pub e: E293, pub f: F293, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type293 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type293{sub: Default::default(), e: Default::default(), f: F293::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type293 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E294 { A, B, C }
impl Default for E294 { fn default() -> E294 { E294::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F294 { A, B, C }
impl Default for F294 { fn default() -> F294 { F294::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub294 { pub e: E294, pub f: F294, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub294 { pub fn new(name: String) -> Sub294 { Sub294{e: Default::default(), f: F294::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type294 { pub sub: Sub294, pub e: E294, pub f: F294, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type294 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type294{sub: Default::default(), e: Default::default(), f: F294::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type294 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E295 { A, B, C }
impl Default for E295 { fn default() -> E295 { E295::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F295 { A, B, C }
impl Default for F295 { fn default() -> F295 { F295::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub295 { pub e: E295, pub f: F295, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub295 { pub fn new(name: String) -> Sub295 { Sub295{e: Default::default(), f: F295::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type295 { pub sub: Sub295, pub e: E295, pub f: F295, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type295 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type295{sub: Default::default(), e: Default::default(), f: F295::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type295 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E296 { A, B, C }
impl Default for E296 { fn default() -> E296 { E296::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F296 { A, B, C }
impl Default for F296 { fn default() -> F296 { F296::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub296 { pub e: E296, pub f: F296, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub296 { pub fn new(name: String) -> Sub296 { Sub296{e: Default::default(), f: F296::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type296 { pub sub: Sub296, pub e: E296, pub f: F296, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type296 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type296{sub: Default::default(), e: Default::default(), f: F296::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type296 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E297 { A, B, C }
impl Default for E297 { fn default() -> E297 { E297::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F297 { A, B, C }
impl Default for F297 { fn default() -> F297 { F297::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub297 { pub e: E297, pub f: F297, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub297 { pub fn new(name: String) -> Sub297 { Sub297{e: Default::default(), f: F297::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type297 { pub sub: Sub297, pub e: E297, pub f: F297, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type297 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type297{sub: Default::default(), e: Default::default(), f: F297::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type297 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E298 { A, B, C }
impl Default for E298 { fn default() -> E298 { E298::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F298 { A, B, C }
impl Default for F298 { fn default() -> F298 { F298::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub298 { pub e: E298, pub f: F298, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub298 { pub fn new(name: String) -> Sub298 { Sub298{e: Default::default(), f: F298::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type298 { pub sub: Sub298, pub e: E298, pub f: F298, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type298 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type298{sub: Default::default(), e: Default::default(), f: F298::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type298 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E299 { A, B, C }
impl Default for E299 { fn default() -> E299 { E299::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F299 { A, B, C }
impl Default for F299 { fn default() -> F299 { F299::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub299 { pub e: E299, pub f: F299, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub299 { pub fn new(name: String) -> Sub299 { Sub299{e: Default::default(), f: F299::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type299 { pub sub: Sub299, pub e: E299, pub f: F299, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type299 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type299{sub: Default::default(), e: Default::default(), f: F299::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type299 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E300 { A, B, C }
impl Default for E300 { fn default() -> E300 { E300::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F300 { A, B, C }
impl Default for F300 { fn default() -> F300 { F300::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub300 { pub e: E300, pub f: F300, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub300 { pub fn new(name: String) -> Sub300 { Sub300{e: Default::default(), f: F300::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type300 { pub sub: Sub300, pub e: E300, pub f: F300, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type300 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type300{sub: Default::default(), e: Default::default(), f: F300::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type300 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E301 { A, B, C }
impl Default for E301 { fn default() -> E301 { E301::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F301 { A, B, C }
impl Default for F301 { fn default() -> F301 { F301::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub301 { pub e: E301, pub f: F301, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub301 { pub fn new(name: String) -> Sub301 { Sub301{e: Default::default(), f: F301::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type301 { pub sub: Sub301, pub e: E301, pub f: F301, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type301 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type301{sub: Default::default(), e: Default::default(), f: F301::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type301 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E302 { A, B, C }
impl Default for E302 { fn default() -> E302 { E302::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F302 { A, B, C }
impl Default for F302 { fn default() -> F302 { F302::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub302 { pub e: E302, pub f: F302, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub302 { pub fn new(name: String) -> Sub302 { Sub302{e: Default::default(), f: F302::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type302 { pub sub: Sub302, pub e: E302, pub f: F302, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type302 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type302{sub: Default::default(), e: Default::default(), f: F302::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type302 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E303 { A, B, C }
impl Default for E303 { fn default() -> E303 { E303::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F303 { A, B, C }
impl Default for F303 { fn default() -> F303 { F303::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub303 { pub e: E303, pub f: F303, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub303 { pub fn new(name: String) -> Sub303 { Sub303{e: Default::default(), f: F303::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type303 { pub sub: Sub303, pub e: E303, pub f: F303, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type303 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type303{sub: Default::default(), e: Default::default(), f: F303::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type303 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E304 { A, B, C }
impl Default for E304 { fn default() -> E304 { E304::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F304 { A, B, C }
impl Default for F304 { fn default() -> F304 { F304::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub304 { pub e: E304, pub f: F304, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub304 { pub fn new(name: String) -> Sub304 { Sub304{e: Default::default(), f: F304::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type304 { pub sub: Sub304, pub e: E304, pub f: F304, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type304 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type304{sub: Default::default(), e: Default::default(), f: F304::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type304 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E305 { A, B, C }
impl Default for E305 { fn default() -> E305 { E305::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F305 { A, B, C }
impl Default for F305 { fn default() -> F305 { F305::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub305 { pub e: E305, pub f: F305, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub305 { pub fn new(name: String) -> Sub305 { Sub305{e: Default::default(), f: F305::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type305 { pub sub: Sub305, pub e: E305, pub f: F305, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type305 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type305{sub: Default::default(), e: Default::default(), f: F305::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type305 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E306 { A, B, C }
impl Default for E306 { fn default() -> E306 { E306::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F306 { A, B, C }
impl Default for F306 { fn default() -> F306 { F306::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub306 { pub e: E306, pub f: F306, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub306 { pub fn new(name: String) -> Sub306 { Sub306{e: Default::default(), f: F306::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type306 { pub sub: Sub306, pub e: E306, pub f: F306, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type306 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type306{sub: Default::default(), e: Default::default(), f: F306::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type306 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E307 { A, B, C }
impl Default for E307 { fn default() -> E307 { E307::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F307 { A, B, C }
impl Default for F307 { fn default() -> F307 { F307::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub307 { pub e: E307, pub f: F307, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub307 { pub fn new(name: String) -> Sub307 { Sub307{e: Default::default(), f: F307::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type307 { pub sub: Sub307, pub e: E307, pub f: F307, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type307 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type307{sub: Default::default(), e: Default::default(), f: F307::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type307 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E308 { A, B, C }
impl Default for E308 { fn default() -> E308 { E308::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F308 { A, B, C }
impl Default for F308 { fn default() -> F308 { F308::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub308 { pub e: E308, pub f: F308, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub308 { pub fn new(name: String) -> Sub308 { Sub308{e: Default::default(), f: F308::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type308 { pub sub: Sub308, pub e: E308, pub f: F308, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type308 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type308{sub: Default::default(), e: Default::default(), f: F308::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type308 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E309 { A, B, C }
impl Default for E309 { fn default() -> E309 { E309::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F309 { A, B, C }
impl Default for F309 { fn default() -> F309 { F309::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub309 { pub e: E309, pub f: F309, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub309 { pub fn new(name: String) -> Sub309 { Sub309{e: Default::default(), f: F309::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type309 { pub sub: Sub309, pub e: E309, pub f: F309, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type309 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type309{sub: Default::default(), e: Default::default(), f: F309::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type309 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E310 { A, B, C }
impl Default for E310 { fn default() -> E310 { E310::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F310 { A, B, C }
impl Default for F310 { fn default() -> F310 { F310::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub310 { pub e: E310, pub f: F310, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub310 { pub fn new(name: String) -> Sub310 { Sub310{e: Default::default(), f: F310::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type310 { pub sub: Sub310, pub e: E310, pub f: F310, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type310 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type310{sub: Default::default(), e: Default::default(), f: F310::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type310 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E311 { A, B, C }
impl Default for E311 { fn default() -> E311 { E311::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F311 { A, B, C }
impl Default for F311 { fn default() -> F311 { F311::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub311 { pub e: E311, pub f: F311, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub311 { pub fn new(name: String) -> Sub311 { Sub311{e: Default::default(), f: F311::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type311 { pub sub: Sub311, pub e: E311, pub f: F311, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type311 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type311{sub: Default::default(), e: Default::default(), f: F311::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type311 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E312 { A, B, C }
impl Default for E312 { fn default() -> E312 { E312::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F312 { A, B, C }
impl Default for F312 { fn default() -> F312 { F312::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub312 { pub e: E312, pub f: F312, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub312 { pub fn new(name: String) -> Sub312 { Sub312{e: Default::default(), f: F312::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type312 { pub sub: Sub312, pub e: E312, pub f: F312, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type312 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type312{sub: Default::default(), e: Default::default(), f: F312::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type312 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E313 { A, B, C }
impl Default for E313 { fn default() -> E313 { E313::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F313 { A, B, C }
impl Default for F313 { fn default() -> F313 { F313::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub313 { pub e: E313, pub f: F313, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub313 { pub fn new(name: String) -> Sub313 { Sub313{e: Default::default(), f: F313::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type313 { pub sub: Sub313, pub e: E313, pub f: F313, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type313 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type313{sub: Default::default(), e: Default::default(), f: F313::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type313 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E314 { A, B, C }
impl Default for E314 { fn default() -> E314 { E314::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F314 { A, B, C }
impl Default for F314 { fn default() -> F314 { F314::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub314 { pub e: E314, pub f: F314, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub314 { pub fn new(name: String) -> Sub314 { Sub314{e: Default::default(), f: F314::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type314 { pub sub: Sub314, pub e: E314, pub f: F314, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type314 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type314{sub: Default::default(), e: Default::default(), f: F314::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type314 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E315 { A, B, C }
impl Default for E315 { fn default() -> E315 { E315::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F315 { A, B, C }
impl Default for F315 { fn default() -> F315 { F315::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub315 { pub e: E315, pub f: F315, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub315 { pub fn new(name: String) -> Sub315 { Sub315{e: Default::default(), f: F315::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type315 { pub sub: Sub315, pub e: E315, pub f: F315, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type315 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type315{sub: Default::default(), e: Default::default(), f: F315::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type315 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E316 { A, B, C }
impl Default for E316 { fn default() -> E316 { E316::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F316 { A, B, C }
impl Default for F316 { fn default() -> F316 { F316::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub316 { pub e: E316, pub f: F316, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub316 { pub fn new(name: String) -> Sub316 { Sub316{e: Default::default(), f: F316::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type316 { pub sub: Sub316, pub e: E316, pub f: F316, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type316 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type316{sub: Default::default(), e: Default::default(), f: F316::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type316 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E317 { A, B, C }
impl Default for E317 { fn default() -> E317 { E317::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F317 { A, B, C }
impl Default for F317 { fn default() -> F317 { F317::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub317 { pub e: E317, pub f: F317, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub317 { pub fn new(name: String) -> Sub317 { Sub317{e: Default::default(), f: F317::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type317 { pub sub: Sub317, pub e: E317, pub f: F317, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type317 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type317{sub: Default::default(), e: Default::default(), f: F317::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type317 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E318 { A, B, C }
impl Default for E318 { fn default() -> E318 { E318::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F318 { A, B, C }
impl Default for F318 { fn default() -> F318 { F318::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub318 { pub e: E318, pub f: F318, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub318 { pub fn new(name: String) -> Sub318 { Sub318{e: Default::default(), f: F318::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type318 { pub sub: Sub318, pub e: E318, pub f: F318, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type318 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type318{sub: Default::default(), e: Default::default(), f: F318::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type318 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E319 { A, B, C }
impl Default for E319 { fn default() -> E319 { E319::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F319 { A, B, C }
impl Default for F319 { fn default() -> F319 { F319::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub319 { pub e: E319, pub f: F319, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub319 { pub fn new(name: String) -> Sub319 { Sub319{e: Default::default(), f: F319::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type319 { pub sub: Sub319, pub e: E319, pub f: F319, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type319 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type319{sub: Default::default(), e: Default::default(), f: F319::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type319 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E320 { A, B, C }
impl Default for E320 { fn default() -> E320 { E320::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F320 { A, B, C }
impl Default for F320 { fn default() -> F320 { F320::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub320 { pub e: E320, pub f: F320, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub320 { pub fn new(name: String) -> Sub320 { Sub320{e: Default::default(), f: F320::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type320 { pub sub: Sub320, pub e: E320, pub f: F320, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type320 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type320{sub: Default::default(), e: Default::default(), f: F320::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type320 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E321 { A, B, C }
impl Default for E321 { fn default() -> E321 { E321::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F321 { A, B, C }
impl Default for F321 { fn default() -> F321 { F321::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub321 { pub e: E321, pub f: F321, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub321 { pub fn new(name: String) -> Sub321 { Sub321{e: Default::default(), f: F321::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type321 { pub sub: Sub321, pub e: E321, pub f: F321, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type321 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type321{sub: Default::default(), e: Default::default(), f: F321::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type321 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E322 { A, B, C }
impl Default for E322 { fn default() -> E322 { E322::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F322 { A, B, C }
impl Default for F322 { fn default() -> F322 { F322::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub322 { pub e: E322, pub f: F322, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub322 { pub fn new(name: String) -> Sub322 { Sub322{e: Default::default(), f: F322::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type322 { pub sub: Sub322, pub e: E322, pub f: F322, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type322 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type322{sub: Default::default(), e: Default::default(), f: F322::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type322 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E323 { A, B, C }
impl Default for E323 { fn default() -> E323 { E323::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F323 { A, B, C }
impl Default for F323 { fn default() -> F323 { F323::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub323 { pub e: E323, pub f: F323, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub323 { pub fn new(name: String) -> Sub323 { Sub323{e: Default::default(), f: F323::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type323 { pub sub: Sub323, pub e: E323, pub f: F323, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type323 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type323{sub: Default::default(), e: Default::default(), f: F323::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type323 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E324 { A, B, C }
impl Default for E324 { fn default() -> E324 { E324::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F324 { A, B, C }
impl Default for F324 { fn default() -> F324 { F324::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub324 { pub e: E324, pub f: F324, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub324 { pub fn new(name: String) -> Sub324 { Sub324{e: Default::default(), f: F324::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type324 { pub sub: Sub324, pub e: E324, pub f: F324, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type324 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type324{sub: Default::default(), e: Default::default(), f: F324::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type324 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E325 { A, B, C }
impl Default for E325 { fn default() -> E325 { E325::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F325 { A, B, C }
impl Default for F325 { fn default() -> F325 { F325::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub325 { pub e: E325, pub f: F325, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub325 { pub fn new(name: String) -> Sub325 { Sub325{e: Default::default(), f: F325::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type325 { pub sub: Sub325, pub e: E325, pub f: F325, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type325 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type325{sub: Default::default(), e: Default::default(), f: F325::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type325 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E326 { A, B, C }
impl Default for E326 { fn default() -> E326 { E326::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F326 { A, B, C }
impl Default for F326 { fn default() -> F326 { F326::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub326 { pub e: E326, pub f: F326, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub326 { pub fn new(name: String) -> Sub326 { Sub326{e: Default::default(), f: F326::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type326 { pub sub: Sub326, pub e: E326, pub f: F326, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type326 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type326{sub: Default::default(), e: Default::default(), f: F326::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type326 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E327 { A, B, C }
impl Default for E327 { fn default() -> E327 { E327::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F327 { A, B, C }
impl Default for F327 { fn default() -> F327 { F327::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub327 { pub e: E327, pub f: F327, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub327 { pub fn new(name: String) -> Sub327 { Sub327{e: Default::default(), f: F327::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type327 { pub sub: Sub327, pub e: E327, pub f: F327, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type327 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type327{sub: Default::default(), e: Default::default(), f: F327::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type327 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E328 { A, B, C }
impl Default for E328 { fn default() -> E328 { E328::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F328 { A, B, C }
impl Default for F328 { fn default() -> F328 { F328::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub328 { pub e: E328, pub f: F328, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub328 { pub fn new(name: String) -> Sub328 { Sub328{e: Default::default(), f: F328::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type328 { pub sub: Sub328, pub e: E328, pub f: F328, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type328 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type328{sub: Default::default(), e: Default::default(), f: F328::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type328 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E329 { A, B, C }
impl Default for E329 { fn default() -> E329 { E329::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F329 { A, B, C }
impl Default for F329 { fn default() -> F329 { F329::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub329 { pub e: E329, pub f: F329, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub329 { pub fn new(name: String) -> Sub329 { Sub329{e: Default::default(), f: F329::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type329 { pub sub: Sub329, pub e: E329, pub f: F329, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type329 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type329{sub: Default::default(), e: Default::default(), f: F329::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type329 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E330 { A, B, C }
impl Default for E330 { fn default() -> E330 { E330::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F330 { A, B, C }
impl Default for F330 { fn default() -> F330 { F330::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub330 { pub e: E330, pub f: F330, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub330 { pub fn new(name: String) -> Sub330 { Sub330{e: Default::default(), f: F330::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type330 { pub sub: Sub330, pub e: E330, pub f: F330, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type330 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type330{sub: Default::default(), e: Default::default(), f: F330::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type330 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E331 { A, B, C }
impl Default for E331 { fn default() -> E331 { E331::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F331 { A, B, C }
impl Default for F331 { fn default() -> F331 { F331::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub331 { pub e: E331, pub f: F331, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub331 { pub fn new(name: String) -> Sub331 { Sub331{e: Default::default(), f: F331::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type331 { pub sub: Sub331, pub e: E331, pub f: F331, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type331 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type331{sub: Default::default(), e: Default::default(), f: F331::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type331 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E332 { A, B, C }
impl Default for E332 { fn default() -> E332 { E332::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F332 { A, B, C }
impl Default for F332 { fn default() -> F332 { F332::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub332 { pub e: E332, pub f: F332, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub332 { pub fn new(name: String) -> Sub332 { Sub332{e: Default::default(), f: F332::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type332 { pub sub: Sub332, pub e: E332, pub f: F332, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type332 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type332{sub: Default::default(), e: Default::default(), f: F332::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type332 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E333 { A, B, C }
impl Default for E333 { fn default() -> E333 { E333::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F333 { A, B, C }
impl Default for F333 { fn default() -> F333 { F333::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub333 { pub e: E333, pub f: F333, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub333 { pub fn new(name: String) -> Sub333 { Sub333{e: Default::default(), f: F333::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type333 { pub sub: Sub333, pub e: E333, pub f: F333, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type333 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type333{sub: Default::default(), e: Default::default(), f: F333::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type333 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E334 { A, B, C }
impl Default for E334 { fn default() -> E334 { E334::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F334 { A, B, C }
impl Default for F334 { fn default() -> F334 { F334::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub334 { pub e: E334, pub f: F334, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub334 { pub fn new(name: String) -> Sub334 { Sub334{e: Default::default(), f: F334::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type334 { pub sub: Sub334, pub e: E334, pub f: F334, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type334 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type334{sub: Default::default(), e: Default::default(), f: F334::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type334 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E335 { A, B, C }
impl Default for E335 { fn default() -> E335 { E335::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F335 { A, B, C }
impl Default for F335 { fn default() -> F335 { F335::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub335 { pub e: E335, pub f: F335, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub335 { pub fn new(name: String) -> Sub335 { Sub335{e: Default::default(), f: F335::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type335 { pub sub: Sub335, pub e: E335, pub f: F335, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type335 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type335{sub: Default::default(), e: Default::default(), f: F335::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type335 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E336 { A, B, C }
impl Default for E336 { fn default() -> E336 { E336::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F336 { A, B, C }
impl Default for F336 { fn default() -> F336 { F336::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub336 { pub e: E336, pub f: F336, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub336 { pub fn new(name: String) -> Sub336 { Sub336{e: Default::default(), f: F336::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type336 { pub sub: Sub336, pub e: E336, pub f: F336, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type336 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type336{sub: Default::default(), e: Default::default(), f: F336::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type336 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E337 { A, B, C }
impl Default for E337 { fn default() -> E337 { E337::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F337 { A, B, C }
impl Default for F337 { fn default() -> F337 { F337::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub337 { pub e: E337, pub f: F337, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub337 { pub fn new(name: String) -> Sub337 { Sub337{e: Default::default(), f: F337::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type337 { pub sub: Sub337, pub e: E337, pub f: F337, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type337 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type337{sub: Default::default(), e: Default::default(), f: F337::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type337 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E338 { A, B, C }
impl Default for E338 { fn default() -> E338 { E338::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F338 { A, B, C }
impl Default for F338 { fn default() -> F338 { F338::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub338 { pub e: E338, pub f: F338, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub338 { pub fn new(name: String) -> Sub338 { Sub338{e: Default::default(), f: F338::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type338 { pub sub: Sub338, pub e: E338, pub f: F338, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type338 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type338{sub: Default::default(), e: Default::default(), f: F338::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type338 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E339 { A, B, C }
impl Default for E339 { fn default() -> E339 { E339::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F339 { A, B, C }
impl Default for F339 { fn default() -> F339 { F339::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub339 { pub e: E339, pub f: F339, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub339 { pub fn new(name: String) -> Sub339 { Sub339{e: Default::default(), f: F339::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type339 { pub sub: Sub339, pub e: E339, pub f: F339, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type339 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type339{sub: Default::default(), e: Default::default(), f: F339::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type339 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E340 { A, B, C }
impl Default for E340 { fn default() -> E340 { E340::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F340 { A, B, C }
impl Default for F340 { fn default() -> F340 { F340::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub340 { pub e: E340, pub f: F340, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub340 { pub fn new(name: String) -> Sub340 { Sub340{e: Default::default(), f: F340::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type340 { pub sub: Sub340, pub e: E340, pub f: F340, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type340 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type340{sub: Default::default(), e: Default::default(), f: F340::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type340 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E341 { A, B, C }
impl Default for E341 { fn default() -> E341 { E341::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F341 { A, B, C }
impl Default for F341 { fn default() -> F341 { F341::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub341 { pub e: E341, pub f: F341, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub341 { pub fn new(name: String) -> Sub341 { Sub341{e: Default::default(), f: F341::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type341 { pub sub: Sub341, pub e: E341, pub f: F341, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type341 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type341{sub: Default::default(), e: Default::default(), f: F341::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type341 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E342 { A, B, C }
impl Default for E342 { fn default() -> E342 { E342::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F342 { A, B, C }
impl Default for F342 { fn default() -> F342 { F342::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub342 { pub e: E342, pub f: F342, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub342 { pub fn new(name: String) -> Sub342 { Sub342{e: Default::default(), f: F342::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type342 { pub sub: Sub342, pub e: E342, pub f: F342, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type342 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type342{sub: Default::default(), e: Default::default(), f: F342::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type342 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E343 { A, B, C }
impl Default for E343 { fn default() -> E343 { E343::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F343 { A, B, C }
impl Default for F343 { fn default() -> F343 { F343::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub343 { pub e: E343, pub f: F343, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub343 { pub fn new(name: String) -> Sub343 { Sub343{e: Default::default(), f: F343::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type343 { pub sub: Sub343, pub e: E343, pub f: F343, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type343 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type343{sub: Default::default(), e: Default::default(), f: F343::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type343 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E344 { A, B, C }
impl Default for E344 { fn default() -> E344 { E344::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F344 { A, B, C }
impl Default for F344 { fn default() -> F344 { F344::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub344 { pub e: E344, pub f: F344, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub344 { pub fn new(name: String) -> Sub344 { Sub344{e: Default::default(), f: F344::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type344 { pub sub: Sub344, pub e: E344, pub f: F344, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type344 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type344{sub: Default::default(), e: Default::default(), f: F344::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type344 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E345 { A, B, C }
impl Default for E345 { fn default() -> E345 { E345::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F345 { A, B, C }
impl Default for F345 { fn default() -> F345 { F345::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub345 { pub e: E345, pub f: F345, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub345 { pub fn new(name: String) -> Sub345 { Sub345{e: Default::default(), f: F345::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type345 { pub sub: Sub345, pub e: E345, pub f: F345, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type345 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type345{sub: Default::default(), e: Default::default(), f: F345::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type345 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E346 { A, B, C }
impl Default for E346 { fn default() -> E346 { E346::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F346 { A, B, C }
impl Default for F346 { fn default() -> F346 { F346::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub346 { pub e: E346, pub f: F346, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub346 { pub fn new(name: String) -> Sub346 { Sub346{e: Default::default(), f: F346::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type346 { pub sub: Sub346, pub e: E346, pub f: F346, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type346 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type346{sub: Default::default(), e: Default::default(), f: F346::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type346 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E347 { A, B, C }
impl Default for E347 { fn default() -> E347 { E347::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F347 { A, B, C }
impl Default for F347 { fn default() -> F347 { F347::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub347 { pub e: E347, pub f: F347, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub347 { pub fn new(name: String) -> Sub347 { Sub347{e: Default::default(), f: F347::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type347 { pub sub: Sub347, pub e: E347, pub f: F347, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type347 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type347{sub: Default::default(), e: Default::default(), f: F347::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type347 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E348 { A, B, C }
impl Default for E348 { fn default() -> E348 { E348::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F348 { A, B, C }
impl Default for F348 { fn default() -> F348 { F348::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub348 { pub e: E348, pub f: F348, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub348 { pub fn new(name: String) -> Sub348 { Sub348{e: Default::default(), f: F348::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type348 { pub sub: Sub348, pub e: E348, pub f: F348, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type348 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type348{sub: Default::default(), e: Default::default(), f: F348::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type348 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E349 { A, B, C }
impl Default for E349 { fn default() -> E349 { E349::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F349 { A, B, C }
impl Default for F349 { fn default() -> F349 { F349::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub349 { pub e: E349, pub f: F349, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub349 { pub fn new(name: String) -> Sub349 { Sub349{e: Default::default(), f: F349::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type349 { pub sub: Sub349, pub e: E349, pub f: F349, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type349 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type349{sub: Default::default(), e: Default::default(), f: F349::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type349 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E350 { A, B, C }
impl Default for E350 { fn default() -> E350 { E350::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F350 { A, B, C }
impl Default for F350 { fn default() -> F350 { F350::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub350 { pub e: E350, pub f: F350, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub350 { pub fn new(name: String) -> Sub350 { Sub350{e: Default::default(), f: F350::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type350 { pub sub: Sub350, pub e: E350, pub f: F350, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type350 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type350{sub: Default::default(), e: Default::default(), f: F350::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type350 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E351 { A, B, C }
impl Default for E351 { fn default() -> E351 { E351::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F351 { A, B, C }
impl Default for F351 { fn default() -> F351 { F351::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub351 { pub e: E351, pub f: F351, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub351 { pub fn new(name: String) -> Sub351 { Sub351{e: Default::default(), f: F351::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type351 { pub sub: Sub351, pub e: E351, pub f: F351, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type351 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type351{sub: Default::default(), e: Default::default(), f: F351::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type351 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E352 { A, B, C }
impl Default for E352 { fn default() -> E352 { E352::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F352 { A, B, C }
impl Default for F352 { fn default() -> F352 { F352::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub352 { pub e: E352, pub f: F352, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub352 { pub fn new(name: String) -> Sub352 { Sub352{e: Default::default(), f: F352::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type352 { pub sub: Sub352, pub e: E352, pub f: F352, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type352 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type352{sub: Default::default(), e: Default::default(), f: F352::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type352 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E353 { A, B, C }
impl Default for E353 { fn default() -> E353 { E353::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F353 { A, B, C }
impl Default for F353 { fn default() -> F353 { F353::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub353 { pub e: E353, pub f: F353, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub353 { pub fn new(name: String) -> Sub353 { Sub353{e: Default::default(), f: F353::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type353 { pub sub: Sub353, pub e: E353, pub f: F353, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type353 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type353{sub: Default::default(), e: Default::default(), f: F353::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type353 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E354 { A, B, C }
impl Default for E354 { fn default() -> E354 { E354::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F354 { A, B, C }
impl Default for F354 { fn default() -> F354 { F354::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub354 { pub e: E354, pub f: F354, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub354 { pub fn new(name: String) -> Sub354 { Sub354{e: Default::default(), f: F354::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type354 { pub sub: Sub354, pub e: E354, pub f: F354, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type354 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type354{sub: Default::default(), e: Default::default(), f: F354::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type354 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E355 { A, B, C }
impl Default for E355 { fn default() -> E355 { E355::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F355 { A, B, C }
impl Default for F355 { fn default() -> F355 { F355::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub355 { pub e: E355, pub f: F355, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub355 { pub fn new(name: String) -> Sub355 { Sub355{e: Default::default(), f: F355::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type355 { pub sub: Sub355, pub e: E355, pub f: F355, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type355 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type355{sub: Default::default(), e: Default::default(), f: F355::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type355 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E356 { A, B, C }
impl Default for E356 { fn default() -> E356 { E356::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F356 { A, B, C }
impl Default for F356 { fn default() -> F356 { F356::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub356 { pub e: E356, pub f: F356, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub356 { pub fn new(name: String) -> Sub356 { Sub356{e: Default::default(), f: F356::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type356 { pub sub: Sub356, pub e: E356, pub f: F356, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type356 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type356{sub: Default::default(), e: Default::default(), f: F356::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type356 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E357 { A, B, C }
impl Default for E357 { fn default() -> E357 { E357::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F357 { A, B, C }
impl Default for F357 { fn default() -> F357 { F357::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub357 { pub e: E357, pub f: F357, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub357 { pub fn new(name: String) -> Sub357 { Sub357{e: Default::default(), f: F357::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type357 { pub sub: Sub357, pub e: E357, pub f: F357, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type357 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type357{sub: Default::default(), e: Default::default(), f: F357::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type357 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E358 { A, B, C }
impl Default for E358 { fn default() -> E358 { E358::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F358 { A, B, C }
impl Default for F358 { fn default() -> F358 { F358::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub358 { pub e: E358, pub f: F358, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub358 { pub fn new(name: String) -> Sub358 { Sub358{e: Default::default(), f: F358::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type358 { pub sub: Sub358, pub e: E358, pub f: F358, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type358 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type358{sub: Default::default(), e: Default::default(), f: F358::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type358 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E359 { A, B, C }
impl Default for E359 { fn default() -> E359 { E359::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F359 { A, B, C }
impl Default for F359 { fn default() -> F359 { F359::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub359 { pub e: E359, pub f: F359, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub359 { pub fn new(name: String) -> Sub359 { Sub359{e: Default::default(), f: F359::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type359 { pub sub: Sub359, pub e: E359, pub f: F359, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type359 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type359{sub: Default::default(), e: Default::default(), f: F359::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type359 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E360 { A, B, C }
impl Default for E360 { fn default() -> E360 { E360::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F360 { A, B, C }
impl Default for F360 { fn default() -> F360 { F360::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub360 { pub e: E360, pub f: F360, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub360 { pub fn new(name: String) -> Sub360 { Sub360{e: Default::default(), f: F360::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type360 { pub sub: Sub360, pub e: E360, pub f: F360, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type360 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type360{sub: Default::default(), e: Default::default(), f: F360::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type360 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E361 { A, B, C }
impl Default for E361 { fn default() -> E361 { E361::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F361 { A, B, C }
impl Default for F361 { fn default() -> F361 { F361::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub361 { pub e: E361, pub f: F361, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub361 { pub fn new(name: String) -> Sub361 { Sub361{e: Default::default(), f: F361::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type361 { pub sub: Sub361, pub e: E361, pub f: F361, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type361 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type361{sub: Default::default(), e: Default::default(), f: F361::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type361 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E362 { A, B, C }
impl Default for E362 { fn default() -> E362 { E362::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F362 { A, B, C }
impl Default for F362 { fn default() -> F362 { F362::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub362 { pub e: E362, pub f: F362, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub362 { pub fn new(name: String) -> Sub362 { Sub362{e: Default::default(), f: F362::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type362 { pub sub: Sub362, pub e: E362, pub f: F362, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type362 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type362{sub: Default::default(), e: Default::default(), f: F362::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type362 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E363 { A, B, C }
impl Default for E363 { fn default() -> E363 { E363::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F363 { A, B, C }
impl Default for F363 { fn default() -> F363 { F363::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub363 { pub e: E363, pub f: F363, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub363 { pub fn new(name: String) -> Sub363 { Sub363{e: Default::default(), f: F363::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type363 { pub sub: Sub363, pub e: E363, pub f: F363, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type363 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type363{sub: Default::default(), e: Default::default(), f: F363::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type363 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E364 { A, B, C }
impl Default for E364 { fn default() -> E364 { E364::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F364 { A, B, C }
impl Default for F364 { fn default() -> F364 { F364::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub364 { pub e: E364, pub f: F364, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub364 { pub fn new(name: String) -> Sub364 { Sub364{e: Default::default(), f: F364::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type364 { pub sub: Sub364, pub e: E364, pub f: F364, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type364 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type364{sub: Default::default(), e: Default::default(), f: F364::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type364 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E365 { A, B, C }
impl Default for E365 { fn default() -> E365 { E365::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F365 { A, B, C }
impl Default for F365 { fn default() -> F365 { F365::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub365 { pub e: E365, pub f: F365, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub365 { pub fn new(name: String) -> Sub365 { Sub365{e: Default::default(), f: F365::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type365 { pub sub: Sub365, pub e: E365, pub f: F365, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type365 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type365{sub: Default::default(), e: Default::default(), f: F365::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type365 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E366 { A, B, C }
impl Default for E366 { fn default() -> E366 { E366::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F366 { A, B, C }
impl Default for F366 { fn default() -> F366 { F366::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub366 { pub e: E366, pub f: F366, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub366 { pub fn new(name: String) -> Sub366 { Sub366{e: Default::default(), f: F366::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type366 { pub sub: Sub366, pub e: E366, pub f: F366, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type366 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type366{sub: Default::default(), e: Default::default(), f: F366::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type366 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E367 { A, B, C }
impl Default for E367 { fn default() -> E367 { E367::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F367 { A, B, C }
impl Default for F367 { fn default() -> F367 { F367::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub367 { pub e: E367, pub f: F367, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub367 { pub fn new(name: String) -> Sub367 { Sub367{e: Default::default(), f: F367::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type367 { pub sub: Sub367, pub e: E367, pub f: F367, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type367 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type367{sub: Default::default(), e: Default::default(), f: F367::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type367 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E368 { A, B, C }
impl Default for E368 { fn default() -> E368 { E368::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F368 { A, B, C }
impl Default for F368 { fn default() -> F368 { F368::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub368 { pub e: E368, pub f: F368, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub368 { pub fn new(name: String) -> Sub368 { Sub368{e: Default::default(), f: F368::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type368 { pub sub: Sub368, pub e: E368, pub f: F368, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type368 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type368{sub: Default::default(), e: Default::default(), f: F368::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type368 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E369 { A, B, C }
impl Default for E369 { fn default() -> E369 { E369::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F369 { A, B, C }
impl Default for F369 { fn default() -> F369 { F369::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub369 { pub e: E369, pub f: F369, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub369 { pub fn new(name: String) -> Sub369 { Sub369{e: Default::default(), f: F369::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type369 { pub sub: Sub369, pub e: E369, pub f: F369, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type369 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type369{sub: Default::default(), e: Default::default(), f: F369::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type369 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E370 { A, B, C }
impl Default for E370 { fn default() -> E370 { E370::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F370 { A, B, C }
impl Default for F370 { fn default() -> F370 { F370::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub370 { pub e: E370, pub f: F370, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub370 { pub fn new(name: String) -> Sub370 { Sub370{e: Default::default(), f: F370::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type370 { pub sub: Sub370, pub e: E370, pub f: F370, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type370 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type370{sub: Default::default(), e: Default::default(), f: F370::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type370 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E371 { A, B, C }
impl Default for E371 { fn default() -> E371 { E371::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F371 { A, B, C }
impl Default for F371 { fn default() -> F371 { F371::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub371 { pub e: E371, pub f: F371, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub371 { pub fn new(name: String) -> Sub371 { Sub371{e: Default::default(), f: F371::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type371 { pub sub: Sub371, pub e: E371, pub f: F371, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type371 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type371{sub: Default::default(), e: Default::default(), f: F371::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type371 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E372 { A, B, C }
impl Default for E372 { fn default() -> E372 { E372::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F372 { A, B, C }
impl Default for F372 { fn default() -> F372 { F372::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub372 { pub e: E372, pub f: F372, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub372 { pub fn new(name: String) -> Sub372 { Sub372{e: Default::default(), f: F372::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type372 { pub sub: Sub372, pub e: E372, pub f: F372, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type372 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type372{sub: Default::default(), e: Default::default(), f: F372::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type372 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E373 { A, B, C }
impl Default for E373 { fn default() -> E373 { E373::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F373 { A, B, C }
impl Default for F373 { fn default() -> F373 { F373::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub373 { pub e: E373, pub f: F373, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub373 { pub fn new(name: String) -> Sub373 { Sub373{e: Default::default(), f: F373::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type373 { pub sub: Sub373, pub e: E373, pub f: F373, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type373 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type373{sub: Default::default(), e: Default::default(), f: F373::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type373 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E374 { A, B, C }
impl Default for E374 { fn default() -> E374 { E374::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F374 { A, B, C }
impl Default for F374 { fn default() -> F374 { F374::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub374 { pub e: E374, pub f: F374, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub374 { pub fn new(name: String) -> Sub374 { Sub374{e: Default::default(), f: F374::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type374 { pub sub: Sub374, pub e: E374, pub f: F374, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type374 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type374{sub: Default::default(), e: Default::default(), f: F374::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type374 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E375 { A, B, C }
impl Default for E375 { fn default() -> E375 { E375::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F375 { A, B, C }
impl Default for F375 { fn default() -> F375 { F375::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub375 { pub e: E375, pub f: F375, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub375 { pub fn new(name: String) -> Sub375 { Sub375{e: Default::default(), f: F375::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type375 { pub sub: Sub375, pub e: E375, pub f: F375, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type375 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type375{sub: Default::default(), e: Default::default(), f: F375::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type375 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E376 { A, B, C }
impl Default for E376 { fn default() -> E376 { E376::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F376 { A, B, C }
impl Default for F376 { fn default() -> F376 { F376::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub376 { pub e: E376, pub f: F376, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub376 { pub fn new(name: String) -> Sub376 { Sub376{e: Default::default(), f: F376::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type376 { pub sub: Sub376, pub e: E376, pub f: F376, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type376 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type376{sub: Default::default(), e: Default::default(), f: F376::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type376 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E377 { A, B, C }
impl Default for E377 { fn default() -> E377 { E377::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F377 { A, B, C }
impl Default for F377 { fn default() -> F377 { F377::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub377 { pub e: E377, pub f: F377, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub377 { pub fn new(name: String) -> Sub377 { Sub377{e: Default::default(), f: F377::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type377 { pub sub: Sub377, pub e: E377, pub f: F377, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type377 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type377{sub: Default::default(), e: Default::default(), f: F377::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type377 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E378 { A, B, C }
impl Default for E378 { fn default() -> E378 { E378::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F378 { A, B, C }
impl Default for F378 { fn default() -> F378 { F378::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub378 { pub e: E378, pub f: F378, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub378 { pub fn new(name: String) -> Sub378 { Sub378{e: Default::default(), f: F378::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type378 { pub sub: Sub378, pub e: E378, pub f: F378, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type378 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type378{sub: Default::default(), e: Default::default(), f: F378::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type378 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E379 { A, B, C }
impl Default for E379 { fn default() -> E379 { E379::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F379 { A, B, C }
impl Default for F379 { fn default() -> F379 { F379::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub379 { pub e: E379, pub f: F379, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub379 { pub fn new(name: String) -> Sub379 { Sub379{e: Default::default(), f: F379::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type379 { pub sub: Sub379, pub e: E379, pub f: F379, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type379 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type379{sub: Default::default(), e: Default::default(), f: F379::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type379 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E380 { A, B, C }
impl Default for E380 { fn default() -> E380 { E380::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F380 { A, B, C }
impl Default for F380 { fn default() -> F380 { F380::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub380 { pub e: E380, pub f: F380, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub380 { pub fn new(name: String) -> Sub380 { Sub380{e: Default::default(), f: F380::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type380 { pub sub: Sub380, pub e: E380, pub f: F380, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type380 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type380{sub: Default::default(), e: Default::default(), f: F380::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type380 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E381 { A, B, C }
impl Default for E381 { fn default() -> E381 { E381::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F381 { A, B, C }
impl Default for F381 { fn default() -> F381 { F381::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub381 { pub e: E381, pub f: F381, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub381 { pub fn new(name: String) -> Sub381 { Sub381{e: Default::default(), f: F381::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type381 { pub sub: Sub381, pub e: E381, pub f: F381, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type381 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type381{sub: Default::default(), e: Default::default(), f: F381::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type381 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E382 { A, B, C }
impl Default for E382 { fn default() -> E382 { E382::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F382 { A, B, C }
impl Default for F382 { fn default() -> F382 { F382::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub382 { pub e: E382, pub f: F382, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub382 { pub fn new(name: String) -> Sub382 { Sub382{e: Default::default(), f: F382::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type382 { pub sub: Sub382, pub e: E382, pub f: F382, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type382 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type382{sub: Default::default(), e: Default::default(), f: F382::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type382 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E383 { A, B, C }
impl Default for E383 { fn default() -> E383 { E383::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F383 { A, B, C }
impl Default for F383 { fn default() -> F383 { F383::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub383 { pub e: E383, pub f: F383, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub383 { pub fn new(name: String) -> Sub383 { Sub383{e: Default::default(), f: F383::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type383 { pub sub: Sub383, pub e: E383, pub f: F383, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type383 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type383{sub: Default::default(), e: Default::default(), f: F383::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type383 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E384 { A, B, C }
impl Default for E384 { fn default() -> E384 { E384::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F384 { A, B, C }
impl Default for F384 { fn default() -> F384 { F384::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub384 { pub e: E384, pub f: F384, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub384 { pub fn new(name: String) -> Sub384 { Sub384{e: Default::default(), f: F384::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type384 { pub sub: Sub384, pub e: E384, pub f: F384, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type384 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type384{sub: Default::default(), e: Default::default(), f: F384::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type384 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E385 { A, B, C }
impl Default for E385 { fn default() -> E385 { E385::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F385 { A, B, C }
impl Default for F385 { fn default() -> F385 { F385::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub385 { pub e: E385, pub f: F385, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub385 { pub fn new(name: String) -> Sub385 { Sub385{e: Default::default(), f: F385::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type385 { pub sub: Sub385, pub e: E385, pub f: F385, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type385 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type385{sub: Default::default(), e: Default::default(), f: F385::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type385 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E386 { A, B, C }
impl Default for E386 { fn default() -> E386 { E386::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F386 { A, B, C }
impl Default for F386 { fn default() -> F386 { F386::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub386 { pub e: E386, pub f: F386, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub386 { pub fn new(name: String) -> Sub386 { Sub386{e: Default::default(), f: F386::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type386 { pub sub: Sub386, pub e: E386, pub f: F386, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type386 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type386{sub: Default::default(), e: Default::default(), f: F386::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type386 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E387 { A, B, C }
impl Default for E387 { fn default() -> E387 { E387::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F387 { A, B, C }
impl Default for F387 { fn default() -> F387 { F387::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub387 { pub e: E387, pub f: F387, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub387 { pub fn new(name: String) -> Sub387 { Sub387{e: Default::default(), f: F387::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type387 { pub sub: Sub387, pub e: E387, pub f: F387, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type387 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type387{sub: Default::default(), e: Default::default(), f: F387::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type387 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E388 { A, B, C }
impl Default for E388 { fn default() -> E388 { E388::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F388 { A, B, C }
impl Default for F388 { fn default() -> F388 { F388::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub388 { pub e: E388, pub f: F388, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub388 { pub fn new(name: String) -> Sub388 { Sub388{e: Default::default(), f: F388::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type388 { pub sub: Sub388, pub e: E388, pub f: F388, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type388 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type388{sub: Default::default(), e: Default::default(), f: F388::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type388 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E389 { A, B, C }
impl Default for E389 { fn default() -> E389 { E389::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F389 { A, B, C }
impl Default for F389 { fn default() -> F389 { F389::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub389 { pub e: E389, pub f: F389, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub389 { pub fn new(name: String) -> Sub389 { Sub389{e: Default::default(), f: F389::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type389 { pub sub: Sub389, pub e: E389, pub f: F389, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type389 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type389{sub: Default::default(), e: Default::default(), f: F389::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type389 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E390 { A, B, C }
impl Default for E390 { fn default() -> E390 { E390::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F390 { A, B, C }
impl Default for F390 { fn default() -> F390 { F390::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub390 { pub e: E390, pub f: F390, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub390 { pub fn new(name: String) -> Sub390 { Sub390{e: Default::default(), f: F390::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type390 { pub sub: Sub390, pub e: E390, pub f: F390, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type390 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type390{sub: Default::default(), e: Default::default(), f: F390::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type390 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E391 { A, B, C }
impl Default for E391 { fn default() -> E391 { E391::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F391 { A, B, C }
impl Default for F391 { fn default() -> F391 { F391::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub391 { pub e: E391, pub f: F391, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub391 { pub fn new(name: String) -> Sub391 { Sub391{e: Default::default(), f: F391::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type391 { pub sub: Sub391, pub e: E391, pub f: F391, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type391 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type391{sub: Default::default(), e: Default::default(), f: F391::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type391 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E392 { A, B, C }
impl Default for E392 { fn default() -> E392 { E392::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F392 { A, B, C }
impl Default for F392 { fn default() -> F392 { F392::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub392 { pub e: E392, pub f: F392, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub392 { pub fn new(name: String) -> Sub392 { Sub392{e: Default::default(), f: F392::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type392 { pub sub: Sub392, pub e: E392, pub f: F392, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type392 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type392{sub: Default::default(), e: Default::default(), f: F392::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type392 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E393 { A, B, C }
impl Default for E393 { fn default() -> E393 { E393::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F393 { A, B, C }
impl Default for F393 { fn default() -> F393 { F393::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub393 { pub e: E393, pub f: F393, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub393 { pub fn new(name: String) -> Sub393 { Sub393{e: Default::default(), f: F393::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type393 { pub sub: Sub393, pub e: E393, pub f: F393, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type393 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type393{sub: Default::default(), e: Default::default(), f: F393::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type393 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E394 { A, B, C }
impl Default for E394 { fn default() -> E394 { E394::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F394 { A, B, C }
impl Default for F394 { fn default() -> F394 { F394::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub394 { pub e: E394, pub f: F394, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub394 { pub fn new(name: String) -> Sub394 { Sub394{e: Default::default(), f: F394::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type394 { pub sub: Sub394, pub e: E394, pub f: F394, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type394 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type394{sub: Default::default(), e: Default::default(), f: F394::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type394 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E395 { A, B, C }
impl Default for E395 { fn default() -> E395 { E395::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F395 { A, B, C }
impl Default for F395 { fn default() -> F395 { F395::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub395 { pub e: E395, pub f: F395, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub395 { pub fn new(name: String) -> Sub395 { Sub395{e: Default::default(), f: F395::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type395 { pub sub: Sub395, pub e: E395, pub f: F395, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type395 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type395{sub: Default::default(), e: Default::default(), f: F395::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type395 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E396 { A, B, C }
impl Default for E396 { fn default() -> E396 { E396::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F396 { A, B, C }
impl Default for F396 { fn default() -> F396 { F396::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub396 { pub e: E396, pub f: F396, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub396 { pub fn new(name: String) -> Sub396 { Sub396{e: Default::default(), f: F396::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type396 { pub sub: Sub396, pub e: E396, pub f: F396, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type396 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type396{sub: Default::default(), e: Default::default(), f: F396::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type396 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E397 { A, B, C }
impl Default for E397 { fn default() -> E397 { E397::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F397 { A, B, C }
impl Default for F397 { fn default() -> F397 { F397::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub397 { pub e: E397, pub f: F397, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub397 { pub fn new(name: String) -> Sub397 { Sub397{e: Default::default(), f: F397::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type397 { pub sub: Sub397, pub e: E397, pub f: F397, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type397 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type397{sub: Default::default(), e: Default::default(), f: F397::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type397 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E398 { A, B, C }
impl Default for E398 { fn default() -> E398 { E398::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F398 { A, B, C }
impl Default for F398 { fn default() -> F398 { F398::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub398 { pub e: E398, pub f: F398, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub398 { pub fn new(name: String) -> Sub398 { Sub398{e: Default::default(), f: F398::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type398 { pub sub: Sub398, pub e: E398, pub f: F398, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type398 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type398{sub: Default::default(), e: Default::default(), f: F398::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type398 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E399 { A, B, C }
impl Default for E399 { fn default() -> E399 { E399::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F399 { A, B, C }
impl Default for F399 { fn default() -> F399 { F399::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub399 { pub e: E399, pub f: F399, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub399 { pub fn new(name: String) -> Sub399 { Sub399{e: Default::default(), f: F399::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type399 { pub sub: Sub399, pub e: E399, pub f: F399, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type399 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type399{sub: Default::default(), e: Default::default(), f: F399::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type399 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E400 { A, B, C }
impl Default for E400 { fn default() -> E400 { E400::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F400 { A, B, C }
impl Default for F400 { fn default() -> F400 { F400::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub400 { pub e: E400, pub f: F400, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub400 { pub fn new(name: String) -> Sub400 { Sub400{e: Default::default(), f: F400::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type400 { pub sub: Sub400, pub e: E400, pub f: F400, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type400 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type400{sub: Default::default(), e: Default::default(), f: F400::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type400 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E401 { A, B, C }
impl Default for E401 { fn default() -> E401 { E401::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F401 { A, B, C }
impl Default for F401 { fn default() -> F401 { F401::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub401 { pub e: E401, pub f: F401, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub401 { pub fn new(name: String) -> Sub401 { Sub401{e: Default::default(), f: F401::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type401 { pub sub: Sub401, pub e: E401, pub f: F401, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type401 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type401{sub: Default::default(), e: Default::default(), f: F401::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type401 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E402 { A, B, C }
impl Default for E402 { fn default() -> E402 { E402::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F402 { A, B, C }
impl Default for F402 { fn default() -> F402 { F402::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub402 { pub e: E402, pub f: F402, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub402 { pub fn new(name: String) -> Sub402 { Sub402{e: Default::default(), f: F402::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type402 { pub sub: Sub402, pub e: E402, pub f: F402, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type402 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type402{sub: Default::default(), e: Default::default(), f: F402::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type402 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E403 { A, B, C }
impl Default for E403 { fn default() -> E403 { E403::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F403 { A, B, C }
impl Default for F403 { fn default() -> F403 { F403::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub403 { pub e: E403, pub f: F403, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub403 { pub fn new(name: String) -> Sub403 { Sub403{e: Default::default(), f: F403::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type403 { pub sub: Sub403, pub e: E403, pub f: F403, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type403 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type403{sub: Default::default(), e: Default::default(), f: F403::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type403 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E404 { A, B, C }
impl Default for E404 { fn default() -> E404 { E404::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F404 { A, B, C }
impl Default for F404 { fn default() -> F404 { F404::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub404 { pub e: E404, pub f: F404, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub404 { pub fn new(name: String) -> Sub404 { Sub404{e: Default::default(), f: F404::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type404 { pub sub: Sub404, pub e: E404, pub f: F404, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type404 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type404{sub: Default::default(), e: Default::default(), f: F404::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type404 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E405 { A, B, C }
impl Default for E405 { fn default() -> E405 { E405::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F405 { A, B, C }
impl Default for F405 { fn default() -> F405 { F405::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub405 { pub e: E405, pub f: F405, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub405 { pub fn new(name: String) -> Sub405 { Sub405{e: Default::default(), f: F405::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type405 { pub sub: Sub405, pub e: E405, pub f: F405, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type405 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type405{sub: Default::default(), e: Default::default(), f: F405::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type405 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E406 { A, B, C }
impl Default for E406 { fn default() -> E406 { E406::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F406 { A, B, C }
impl Default for F406 { fn default() -> F406 { F406::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub406 { pub e: E406, pub f: F406, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub406 { pub fn new(name: String) -> Sub406 { Sub406{e: Default::default(), f: F406::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type406 { pub sub: Sub406, pub e: E406, pub f: F406, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type406 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type406{sub: Default::default(), e: Default::default(), f: F406::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type406 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E407 { A, B, C }
impl Default for E407 { fn default() -> E407 { E407::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F407 { A, B, C }
impl Default for F407 { fn default() -> F407 { F407::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub407 { pub e: E407, pub f: F407, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub407 { pub fn new(name: String) -> Sub407 { Sub407{e: Default::default(), f: F407::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type407 { pub sub: Sub407, pub e: E407, pub f: F407, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type407 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type407{sub: Default::default(), e: Default::default(), f: F407::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type407 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E408 { A, B, C }
impl Default for E408 { fn default() -> E408 { E408::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F408 { A, B, C }
impl Default for F408 { fn default() -> F408 { F408::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub408 { pub e: E408, pub f: F408, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub408 { pub fn new(name: String) -> Sub408 { Sub408{e: Default::default(), f: F408::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type408 { pub sub: Sub408, pub e: E408, pub f: F408, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type408 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type408{sub: Default::default(), e: Default::default(), f: F408::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type408 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E409 { A, B, C }
impl Default for E409 { fn default() -> E409 { E409::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F409 { A, B, C }
impl Default for F409 { fn default() -> F409 { F409::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub409 { pub e: E409, pub f: F409, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub409 { pub fn new(name: String) -> Sub409 { Sub409{e: Default::default(), f: F409::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type409 { pub sub: Sub409, pub e: E409, pub f: F409, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type409 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type409{sub: Default::default(), e: Default::default(), f: F409::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type409 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E410 { A, B, C }
impl Default for E410 { fn default() -> E410 { E410::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F410 { A, B, C }
impl Default for F410 { fn default() -> F410 { F410::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub410 { pub e: E410, pub f: F410, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub410 { pub fn new(name: String) -> Sub410 { Sub410{e: Default::default(), f: F410::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type410 { pub sub: Sub410, pub e: E410, pub f: F410, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type410 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type410{sub: Default::default(), e: Default::default(), f: F410::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type410 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E411 { A, B, C }
impl Default for E411 { fn default() -> E411 { E411::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F411 { A, B, C }
impl Default for F411 { fn default() -> F411 { F411::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub411 { pub e: E411, pub f: F411, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub411 { pub fn new(name: String) -> Sub411 { Sub411{e: Default::default(), f: F411::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type411 { pub sub: Sub411, pub e: E411, pub f: F411, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type411 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type411{sub: Default::default(), e: Default::default(), f: F411::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type411 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E412 { A, B, C }
impl Default for E412 { fn default() -> E412 { E412::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F412 { A, B, C }
impl Default for F412 { fn default() -> F412 { F412::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub412 { pub e: E412, pub f: F412, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub412 { pub fn new(name: String) -> Sub412 { Sub412{e: Default::default(), f: F412::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type412 { pub sub: Sub412, pub e: E412, pub f: F412, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type412 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type412{sub: Default::default(), e: Default::default(), f: F412::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type412 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E413 { A, B, C }
impl Default for E413 { fn default() -> E413 { E413::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F413 { A, B, C }
impl Default for F413 { fn default() -> F413 { F413::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub413 { pub e: E413, pub f: F413, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub413 { pub fn new(name: String) -> Sub413 { Sub413{e: Default::default(), f: F413::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type413 { pub sub: Sub413, pub e: E413, pub f: F413, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type413 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type413{sub: Default::default(), e: Default::default(), f: F413::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type413 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E414 { A, B, C }
impl Default for E414 { fn default() -> E414 { E414::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F414 { A, B, C }
impl Default for F414 { fn default() -> F414 { F414::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub414 { pub e: E414, pub f: F414, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub414 { pub fn new(name: String) -> Sub414 { Sub414{e: Default::default(), f: F414::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type414 { pub sub: Sub414, pub e: E414, pub f: F414, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type414 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type414{sub: Default::default(), e: Default::default(), f: F414::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type414 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E415 { A, B, C }
impl Default for E415 { fn default() -> E415 { E415::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F415 { A, B, C }
impl Default for F415 { fn default() -> F415 { F415::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub415 { pub e: E415, pub f: F415, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub415 { pub fn new(name: String) -> Sub415 { Sub415{e: Default::default(), f: F415::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type415 { pub sub: Sub415, pub e: E415, pub f: F415, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type415 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type415{sub: Default::default(), e: Default::default(), f: F415::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type415 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E416 { A, B, C }
impl Default for E416 { fn default() -> E416 { E416::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F416 { A, B, C }
impl Default for F416 { fn default() -> F416 { F416::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub416 { pub e: E416, pub f: F416, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub416 { pub fn new(name: String) -> Sub416 { Sub416{e: Default::default(), f: F416::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type416 { pub sub: Sub416, pub e: E416, pub f: F416, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type416 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type416{sub: Default::default(), e: Default::default(), f: F416::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type416 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E417 { A, B, C }
impl Default for E417 { fn default() -> E417 { E417::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F417 { A, B, C }
impl Default for F417 { fn default() -> F417 { F417::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub417 { pub e: E417, pub f: F417, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub417 { pub fn new(name: String) -> Sub417 { Sub417{e: Default::default(), f: F417::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type417 { pub sub: Sub417, pub e: E417, pub f: F417, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type417 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type417{sub: Default::default(), e: Default::default(), f: F417::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type417 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E418 { A, B, C }
impl Default for E418 { fn default() -> E418 { E418::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F418 { A, B, C }
impl Default for F418 { fn default() -> F418 { F418::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub418 { pub e: E418, pub f: F418, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub418 { pub fn new(name: String) -> Sub418 { Sub418{e: Default::default(), f: F418::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type418 { pub sub: Sub418, pub e: E418, pub f: F418, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type418 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type418{sub: Default::default(), e: Default::default(), f: F418::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type418 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E419 { A, B, C }
impl Default for E419 { fn default() -> E419 { E419::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F419 { A, B, C }
impl Default for F419 { fn default() -> F419 { F419::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub419 { pub e: E419, pub f: F419, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub419 { pub fn new(name: String) -> Sub419 { Sub419{e: Default::default(), f: F419::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type419 { pub sub: Sub419, pub e: E419, pub f: F419, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type419 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type419{sub: Default::default(), e: Default::default(), f: F419::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type419 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E420 { A, B, C }
impl Default for E420 { fn default() -> E420 { E420::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F420 { A, B, C }
impl Default for F420 { fn default() -> F420 { F420::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub420 { pub e: E420, pub f: F420, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub420 { pub fn new(name: String) -> Sub420 { Sub420{e: Default::default(), f: F420::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type420 { pub sub: Sub420, pub e: E420, pub f: F420, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type420 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type420{sub: Default::default(), e: Default::default(), f: F420::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type420 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E421 { A, B, C }
impl Default for E421 { fn default() -> E421 { E421::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F421 { A, B, C }
impl Default for F421 { fn default() -> F421 { F421::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub421 { pub e: E421, pub f: F421, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub421 { pub fn new(name: String) -> Sub421 { Sub421{e: Default::default(), f: F421::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type421 { pub sub: Sub421, pub e: E421, pub f: F421, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type421 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type421{sub: Default::default(), e: Default::default(), f: F421::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type421 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E422 { A, B, C }
impl Default for E422 { fn default() -> E422 { E422::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F422 { A, B, C }
impl Default for F422 { fn default() -> F422 { F422::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub422 { pub e: E422, pub f: F422, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub422 { pub fn new(name: String) -> Sub422 { Sub422{e: Default::default(), f: F422::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type422 { pub sub: Sub422, pub e: E422, pub f: F422, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type422 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type422{sub: Default::default(), e: Default::default(), f: F422::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type422 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E423 { A, B, C }
impl Default for E423 { fn default() -> E423 { E423::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F423 { A, B, C }
impl Default for F423 { fn default() -> F423 { F423::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub423 { pub e: E423, pub f: F423, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub423 { pub fn new(name: String) -> Sub423 { Sub423{e: Default::default(), f: F423::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type423 { pub sub: Sub423, pub e: E423, pub f: F423, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type423 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type423{sub: Default::default(), e: Default::default(), f: F423::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type423 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E424 { A, B, C }
impl Default for E424 { fn default() -> E424 { E424::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F424 { A, B, C }
impl Default for F424 { fn default() -> F424 { F424::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub424 { pub e: E424, pub f: F424, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub424 { pub fn new(name: String) -> Sub424 { Sub424{e: Default::default(), f: F424::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type424 { pub sub: Sub424, pub e: E424, pub f: F424, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type424 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type424{sub: Default::default(), e: Default::default(), f: F424::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type424 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E425 { A, B, C }
impl Default for E425 { fn default() -> E425 { E425::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F425 { A, B, C }
impl Default for F425 { fn default() -> F425 { F425::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub425 { pub e: E425, pub f: F425, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub425 { pub fn new(name: String) -> Sub425 { Sub425{e: Default::default(), f: F425::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type425 { pub sub: Sub425, pub e: E425, pub f: F425, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type425 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type425{sub: Default::default(), e: Default::default(), f: F425::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type425 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E426 { A, B, C }
impl Default for E426 { fn default() -> E426 { E426::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F426 { A, B, C }
impl Default for F426 { fn default() -> F426 { F426::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub426 { pub e: E426, pub f: F426, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub426 { pub fn new(name: String) -> Sub426 { Sub426{e: Default::default(), f: F426::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type426 { pub sub: Sub426, pub e: E426, pub f: F426, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type426 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type426{sub: Default::default(), e: Default::default(), f: F426::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type426 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E427 { A, B, C }
impl Default for E427 { fn default() -> E427 { E427::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F427 { A, B, C }
impl Default for F427 { fn default() -> F427 { F427::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub427 { pub e: E427, pub f: F427, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub427 { pub fn new(name: String) -> Sub427 { Sub427{e: Default::default(), f: F427::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type427 { pub sub: Sub427, pub e: E427, pub f: F427, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type427 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type427{sub: Default::default(), e: Default::default(), f: F427::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type427 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E428 { A, B, C }
impl Default for E428 { fn default() -> E428 { E428::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F428 { A, B, C }
impl Default for F428 { fn default() -> F428 { F428::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub428 { pub e: E428, pub f: F428, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub428 { pub fn new(name: String) -> Sub428 { Sub428{e: Default::default(), f: F428::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type428 { pub sub: Sub428, pub e: E428, pub f: F428, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type428 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type428{sub: Default::default(), e: Default::default(), f: F428::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type428 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E429 { A, B, C }
impl Default for E429 { fn default() -> E429 { E429::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F429 { A, B, C }
impl Default for F429 { fn default() -> F429 { F429::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub429 { pub e: E429, pub f: F429, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub429 { pub fn new(name: String) -> Sub429 { Sub429{e: Default::default(), f: F429::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type429 { pub sub: Sub429, pub e: E429, pub f: F429, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type429 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type429{sub: Default::default(), e: Default::default(), f: F429::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type429 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E430 { A, B, C }
impl Default for E430 { fn default() -> E430 { E430::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F430 { A, B, C }
impl Default for F430 { fn default() -> F430 { F430::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub430 { pub e: E430, pub f: F430, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub430 { pub fn new(name: String) -> Sub430 { Sub430{e: Default::default(), f: F430::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type430 { pub sub: Sub430, pub e: E430, pub f: F430, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type430 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type430{sub: Default::default(), e: Default::default(), f: F430::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type430 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E431 { A, B, C }
impl Default for E431 { fn default() -> E431 { E431::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F431 { A, B, C }
impl Default for F431 { fn default() -> F431 { F431::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub431 { pub e: E431, pub f: F431, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub431 { pub fn new(name: String) -> Sub431 { Sub431{e: Default::default(), f: F431::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type431 { pub sub: Sub431, pub e: E431, pub f: F431, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type431 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type431{sub: Default::default(), e: Default::default(), f: F431::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type431 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E432 { A, B, C }
impl Default for E432 { fn default() -> E432 { E432::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F432 { A, B, C }
impl Default for F432 { fn default() -> F432 { F432::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub432 { pub e: E432, pub f: F432, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub432 { pub fn new(name: String) -> Sub432 { Sub432{e: Default::default(), f: F432::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type432 { pub sub: Sub432, pub e: E432, pub f: F432, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type432 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type432{sub: Default::default(), e: Default::default(), f: F432::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type432 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E433 { A, B, C }
impl Default for E433 { fn default() -> E433 { E433::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F433 { A, B, C }
impl Default for F433 { fn default() -> F433 { F433::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub433 { pub e: E433, pub f: F433, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub433 { pub fn new(name: String) -> Sub433 { Sub433{e: Default::default(), f: F433::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type433 { pub sub: Sub433, pub e: E433, pub f: F433, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type433 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type433{sub: Default::default(), e: Default::default(), f: F433::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type433 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E434 { A, B, C }
impl Default for E434 { fn default() -> E434 { E434::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F434 { A, B, C }
impl Default for F434 { fn default() -> F434 { F434::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub434 { pub e: E434, pub f: F434, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub434 { pub fn new(name: String) -> Sub434 { Sub434{e: Default::default(), f: F434::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type434 { pub sub: Sub434, pub e: E434, pub f: F434, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type434 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type434{sub: Default::default(), e: Default::default(), f: F434::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type434 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E435 { A, B, C }
impl Default for E435 { fn default() -> E435 { E435::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F435 { A, B, C }
impl Default for F435 { fn default() -> F435 { F435::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub435 { pub e: E435, pub f: F435, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub435 { pub fn new(name: String) -> Sub435 { Sub435{e: Default::default(), f: F435::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type435 { pub sub: Sub435, pub e: E435, pub f: F435, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type435 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type435{sub: Default::default(), e: Default::default(), f: F435::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type435 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E436 { A, B, C }
impl Default for E436 { fn default() -> E436 { E436::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F436 { A, B, C }
impl Default for F436 { fn default() -> F436 { F436::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub436 { pub e: E436, pub f: F436, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub436 { pub fn new(name: String) -> Sub436 { Sub436{e: Default::default(), f: F436::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type436 { pub sub: Sub436, pub e: E436, pub f: F436, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type436 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type436{sub: Default::default(), e: Default::default(), f: F436::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type436 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E437 { A, B, C }
impl Default for E437 { fn default() -> E437 { E437::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F437 { A, B, C }
impl Default for F437 { fn default() -> F437 { F437::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub437 { pub e: E437, pub f: F437, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub437 { pub fn new(name: String) -> Sub437 { Sub437{e: Default::default(), f: F437::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type437 { pub sub: Sub437, pub e: E437, pub f: F437, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type437 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type437{sub: Default::default(), e: Default::default(), f: F437::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type437 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E438 { A, B, C }
impl Default for E438 { fn default() -> E438 { E438::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F438 { A, B, C }
impl Default for F438 { fn default() -> F438 { F438::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub438 { pub e: E438, pub f: F438, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub438 { pub fn new(name: String) -> Sub438 { Sub438{e: Default::default(), f: F438::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type438 { pub sub: Sub438, pub e: E438, pub f: F438, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type438 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type438{sub: Default::default(), e: Default::default(), f: F438::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type438 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E439 { A, B, C }
impl Default for E439 { fn default() -> E439 { E439::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F439 { A, B, C }
impl Default for F439 { fn default() -> F439 { F439::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub439 { pub e: E439, pub f: F439, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub439 { pub fn new(name: String) -> Sub439 { Sub439{e: Default::default(), f: F439::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type439 { pub sub: Sub439, pub e: E439, pub f: F439, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type439 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type439{sub: Default::default(), e: Default::default(), f: F439::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type439 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E440 { A, B, C }
impl Default for E440 { fn default() -> E440 { E440::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F440 { A, B, C }
impl Default for F440 { fn default() -> F440 { F440::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub440 { pub e: E440, pub f: F440, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub440 { pub fn new(name: String) -> Sub440 { Sub440{e: Default::default(), f: F440::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type440 { pub sub: Sub440, pub e: E440, pub f: F440, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type440 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type440{sub: Default::default(), e: Default::default(), f: F440::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type440 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E441 { A, B, C }
impl Default for E441 { fn default() -> E441 { E441::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F441 { A, B, C }
impl Default for F441 { fn default() -> F441 { F441::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub441 { pub e: E441, pub f: F441, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub441 { pub fn new(name: String) -> Sub441 { Sub441{e: Default::default(), f: F441::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type441 { pub sub: Sub441, pub e: E441, pub f: F441, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type441 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type441{sub: Default::default(), e: Default::default(), f: F441::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type441 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E442 { A, B, C }
impl Default for E442 { fn default() -> E442 { E442::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F442 { A, B, C }
impl Default for F442 { fn default() -> F442 { F442::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub442 { pub e: E442, pub f: F442, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub442 { pub fn new(name: String) -> Sub442 { Sub442{e: Default::default(), f: F442::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type442 { pub sub: Sub442, pub e: E442, pub f: F442, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type442 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type442{sub: Default::default(), e: Default::default(), f: F442::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type442 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E443 { A, B, C }
impl Default for E443 { fn default() -> E443 { E443::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F443 { A, B, C }
impl Default for F443 { fn default() -> F443 { F443::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub443 { pub e: E443, pub f: F443, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub443 { pub fn new(name: String) -> Sub443 { Sub443{e: Default::default(), f: F443::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type443 { pub sub: Sub443, pub e: E443, pub f: F443, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type443 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type443{sub: Default::default(), e: Default::default(), f: F443::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type443 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E444 { A, B, C }
impl Default for E444 { fn default() -> E444 { E444::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F444 { A, B, C }
impl Default for F444 { fn default() -> F444 { F444::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub444 { pub e: E444, pub f: F444, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub444 { pub fn new(name: String) -> Sub444 { Sub444{e: Default::default(), f: F444::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type444 { pub sub: Sub444, pub e: E444, pub f: F444, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type444 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type444{sub: Default::default(), e: Default::default(), f: F444::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type444 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E445 { A, B, C }
impl Default for E445 { fn default() -> E445 { E445::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F445 { A, B, C }
impl Default for F445 { fn default() -> F445 { F445::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub445 { pub e: E445, pub f: F445, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub445 { pub fn new(name: String) -> Sub445 { Sub445{e: Default::default(), f: F445::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type445 { pub sub: Sub445, pub e: E445, pub f: F445, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type445 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type445{sub: Default::default(), e: Default::default(), f: F445::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type445 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E446 { A, B, C }
impl Default for E446 { fn default() -> E446 { E446::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F446 { A, B, C }
impl Default for F446 { fn default() -> F446 { F446::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub446 { pub e: E446, pub f: F446, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub446 { pub fn new(name: String) -> Sub446 { Sub446{e: Default::default(), f: F446::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type446 { pub sub: Sub446, pub e: E446, pub f: F446, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type446 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type446{sub: Default::default(), e: Default::default(), f: F446::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type446 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E447 { A, B, C }
impl Default for E447 { fn default() -> E447 { E447::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F447 { A, B, C }
impl Default for F447 { fn default() -> F447 { F447::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub447 { pub e: E447, pub f: F447, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub447 { pub fn new(name: String) -> Sub447 { Sub447{e: Default::default(), f: F447::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type447 { pub sub: Sub447, pub e: E447, pub f: F447, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type447 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type447{sub: Default::default(), e: Default::default(), f: F447::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type447 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E448 { A, B, C }
impl Default for E448 { fn default() -> E448 { E448::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F448 { A, B, C }
impl Default for F448 { fn default() -> F448 { F448::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub448 { pub e: E448, pub f: F448, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub448 { pub fn new(name: String) -> Sub448 { Sub448{e: Default::default(), f: F448::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type448 { pub sub: Sub448, pub e: E448, pub f: F448, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type448 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type448{sub: Default::default(), e: Default::default(), f: F448::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type448 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E449 { A, B, C }
impl Default for E449 { fn default() -> E449 { E449::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F449 { A, B, C }
impl Default for F449 { fn default() -> F449 { F449::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub449 { pub e: E449, pub f: F449, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub449 { pub fn new(name: String) -> Sub449 { Sub449{e: Default::default(), f: F449::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type449 { pub sub: Sub449, pub e: E449, pub f: F449, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type449 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type449{sub: Default::default(), e: Default::default(), f: F449::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type449 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E450 { A, B, C }
impl Default for E450 { fn default() -> E450 { E450::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F450 { A, B, C }
impl Default for F450 { fn default() -> F450 { F450::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub450 { pub e: E450, pub f: F450, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub450 { pub fn new(name: String) -> Sub450 { Sub450{e: Default::default(), f: F450::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type450 { pub sub: Sub450, pub e: E450, pub f: F450, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type450 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type450{sub: Default::default(), e: Default::default(), f: F450::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type450 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E451 { A, B, C }
impl Default for E451 { fn default() -> E451 { E451::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F451 { A, B, C }
impl Default for F451 { fn default() -> F451 { F451::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub451 { pub e: E451, pub f: F451, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub451 { pub fn new(name: String) -> Sub451 { Sub451{e: Default::default(), f: F451::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type451 { pub sub: Sub451, pub e: E451, pub f: F451, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type451 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type451{sub: Default::default(), e: Default::default(), f: F451::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type451 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E452 { A, B, C }
impl Default for E452 { fn default() -> E452 { E452::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F452 { A, B, C }
impl Default for F452 { fn default() -> F452 { F452::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub452 { pub e: E452, pub f: F452, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub452 { pub fn new(name: String) -> Sub452 { Sub452{e: Default::default(), f: F452::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type452 { pub sub: Sub452, pub e: E452, pub f: F452, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type452 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type452{sub: Default::default(), e: Default::default(), f: F452::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type452 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E453 { A, B, C }
impl Default for E453 { fn default() -> E453 { E453::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F453 { A, B, C }
impl Default for F453 { fn default() -> F453 { F453::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub453 { pub e: E453, pub f: F453, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub453 { pub fn new(name: String) -> Sub453 { Sub453{e: Default::default(), f: F453::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type453 { pub sub: Sub453, pub e: E453, pub f: F453, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type453 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type453{sub: Default::default(), e: Default::default(), f: F453::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type453 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E454 { A, B, C }
impl Default for E454 { fn default() -> E454 { E454::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F454 { A, B, C }
impl Default for F454 { fn default() -> F454 { F454::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub454 { pub e: E454, pub f: F454, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub454 { pub fn new(name: String) -> Sub454 { Sub454{e: Default::default(), f: F454::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type454 { pub sub: Sub454, pub e: E454, pub f: F454, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type454 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type454{sub: Default::default(), e: Default::default(), f: F454::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type454 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E455 { A, B, C }
impl Default for E455 { fn default() -> E455 { E455::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F455 { A, B, C }
impl Default for F455 { fn default() -> F455 { F455::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub455 { pub e: E455, pub f: F455, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub455 { pub fn new(name: String) -> Sub455 { Sub455{e: Default::default(), f: F455::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type455 { pub sub: Sub455, pub e: E455, pub f: F455, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type455 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type455{sub: Default::default(), e: Default::default(), f: F455::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type455 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E456 { A, B, C }
impl Default for E456 { fn default() -> E456 { E456::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F456 { A, B, C }
impl Default for F456 { fn default() -> F456 { F456::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub456 { pub e: E456, pub f: F456, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub456 { pub fn new(name: String) -> Sub456 { Sub456{e: Default::default(), f: F456::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type456 { pub sub: Sub456, pub e: E456, pub f: F456, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type456 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type456{sub: Default::default(), e: Default::default(), f: F456::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type456 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E457 { A, B, C }
impl Default for E457 { fn default() -> E457 { E457::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F457 { A, B, C }
impl Default for F457 { fn default() -> F457 { F457::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub457 { pub e: E457, pub f: F457, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub457 { pub fn new(name: String) -> Sub457 { Sub457{e: Default::default(), f: F457::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type457 { pub sub: Sub457, pub e: E457, pub f: F457, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type457 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type457{sub: Default::default(), e: Default::default(), f: F457::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type457 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E458 { A, B, C }
impl Default for E458 { fn default() -> E458 { E458::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F458 { A, B, C }
impl Default for F458 { fn default() -> F458 { F458::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub458 { pub e: E458, pub f: F458, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub458 { pub fn new(name: String) -> Sub458 { Sub458{e: Default::default(), f: F458::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type458 { pub sub: Sub458, pub e: E458, pub f: F458, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type458 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type458{sub: Default::default(), e: Default::default(), f: F458::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type458 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E459 { A, B, C }
impl Default for E459 { fn default() -> E459 { E459::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F459 { A, B, C }
impl Default for F459 { fn default() -> F459 { F459::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub459 { pub e: E459, pub f: F459, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub459 { pub fn new(name: String) -> Sub459 { Sub459{e: Default::default(), f: F459::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type459 { pub sub: Sub459, pub e: E459, pub f: F459, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type459 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type459{sub: Default::default(), e: Default::default(), f: F459::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type459 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E460 { A, B, C }
impl Default for E460 { fn default() -> E460 { E460::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F460 { A, B, C }
impl Default for F460 { fn default() -> F460 { F460::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub460 { pub e: E460, pub f: F460, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub460 { pub fn new(name: String) -> Sub460 { Sub460{e: Default::default(), f: F460::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type460 { pub sub: Sub460, pub e: E460, pub f: F460, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type460 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type460{sub: Default::default(), e: Default::default(), f: F460::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type460 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E461 { A, B, C }
impl Default for E461 { fn default() -> E461 { E461::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F461 { A, B, C }
impl Default for F461 { fn default() -> F461 { F461::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub461 { pub e: E461, pub f: F461, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub461 { pub fn new(name: String) -> Sub461 { Sub461{e: Default::default(), f: F461::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type461 { pub sub: Sub461, pub e: E461, pub f: F461, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type461 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type461{sub: Default::default(), e: Default::default(), f: F461::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type461 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E462 { A, B, C }
impl Default for E462 { fn default() -> E462 { E462::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F462 { A, B, C }
impl Default for F462 { fn default() -> F462 { F462::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub462 { pub e: E462, pub f: F462, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub462 { pub fn new(name: String) -> Sub462 { Sub462{e: Default::default(), f: F462::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type462 { pub sub: Sub462, pub e: E462, pub f: F462, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type462 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type462{sub: Default::default(), e: Default::default(), f: F462::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type462 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E463 { A, B, C }
impl Default for E463 { fn default() -> E463 { E463::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F463 { A, B, C }
impl Default for F463 { fn default() -> F463 { F463::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub463 { pub e: E463, pub f: F463, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub463 { pub fn new(name: String) -> Sub463 { Sub463{e: Default::default(), f: F463::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type463 { pub sub: Sub463, pub e: E463, pub f: F463, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type463 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type463{sub: Default::default(), e: Default::default(), f: F463::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type463 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E464 { A, B, C }
impl Default for E464 { fn default() -> E464 { E464::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F464 { A, B, C }
impl Default for F464 { fn default() -> F464 { F464::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub464 { pub e: E464, pub f: F464, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub464 { pub fn new(name: String) -> Sub464 { Sub464{e: Default::default(), f: F464::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type464 { pub sub: Sub464, pub e: E464, pub f: F464, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type464 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type464{sub: Default::default(), e: Default::default(), f: F464::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type464 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E465 { A, B, C }
impl Default for E465 { fn default() -> E465 { E465::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F465 { A, B, C }
impl Default for F465 { fn default() -> F465 { F465::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub465 { pub e: E465, pub f: F465, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub465 { pub fn new(name: String) -> Sub465 { Sub465{e: Default::default(), f: F465::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type465 { pub sub: Sub465, pub e: E465, pub f: F465, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type465 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type465{sub: Default::default(), e: Default::default(), f: F465::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type465 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E466 { A, B, C }
impl Default for E466 { fn default() -> E466 { E466::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F466 { A, B, C }
impl Default for F466 { fn default() -> F466 { F466::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub466 { pub e: E466, pub f: F466, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub466 { pub fn new(name: String) -> Sub466 { Sub466{e: Default::default(), f: F466::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type466 { pub sub: Sub466, pub e: E466, pub f: F466, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type466 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type466{sub: Default::default(), e: Default::default(), f: F466::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type466 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E467 { A, B, C }
impl Default for E467 { fn default() -> E467 { E467::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F467 { A, B, C }
impl Default for F467 { fn default() -> F467 { F467::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub467 { pub e: E467, pub f: F467, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub467 { pub fn new(name: String) -> Sub467 { Sub467{e: Default::default(), f: F467::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type467 { pub sub: Sub467, pub e: E467, pub f: F467, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type467 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type467{sub: Default::default(), e: Default::default(), f: F467::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type467 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E468 { A, B, C }
impl Default for E468 { fn default() -> E468 { E468::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F468 { A, B, C }
impl Default for F468 { fn default() -> F468 { F468::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub468 { pub e: E468, pub f: F468, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub468 { pub fn new(name: String) -> Sub468 { Sub468{e: Default::default(), f: F468::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type468 { pub sub: Sub468, pub e: E468, pub f: F468, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type468 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type468{sub: Default::default(), e: Default::default(), f: F468::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type468 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E469 { A, B, C }
impl Default for E469 { fn default() -> E469 { E469::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F469 { A, B, C }
impl Default for F469 { fn default() -> F469 { F469::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub469 { pub e: E469, pub f: F469, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub469 { pub fn new(name: String) -> Sub469 { Sub469{e: Default::default(), f: F469::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type469 { pub sub: Sub469, pub e: E469, pub f: F469, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type469 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type469{sub: Default::default(), e: Default::default(), f: F469::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type469 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E470 { A, B, C }
impl Default for E470 { fn default() -> E470 { E470::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F470 { A, B, C }
impl Default for F470 { fn default() -> F470 { F470::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub470 { pub e: E470, pub f: F470, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub470 { pub fn new(name: String) -> Sub470 { Sub470{e: Default::default(), f: F470::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type470 { pub sub: Sub470, pub e: E470, pub f: F470, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type470 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type470{sub: Default::default(), e: Default::default(), f: F470::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type470 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E471 { A, B, C }
impl Default for E471 { fn default() -> E471 { E471::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F471 { A, B, C }
impl Default for F471 { fn default() -> F471 { F471::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub471 { pub e: E471, pub f: F471, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub471 { pub fn new(name: String) -> Sub471 { Sub471{e: Default::default(), f: F471::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type471 { pub sub: Sub471, pub e: E471, pub f: F471, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type471 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type471{sub: Default::default(), e: Default::default(), f: F471::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type471 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E472 { A, B, C }
impl Default for E472 { fn default() -> E472 { E472::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F472 { A, B, C }
impl Default for F472 { fn default() -> F472 { F472::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub472 { pub e: E472, pub f: F472, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub472 { pub fn new(name: String) -> Sub472 { Sub472{e: Default::default(), f: F472::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type472 { pub sub: Sub472, pub e: E472, pub f: F472, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type472 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type472{sub: Default::default(), e: Default::default(), f: F472::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type472 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E473 { A, B, C }
impl Default for E473 { fn default() -> E473 { E473::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F473 { A, B, C }
impl Default for F473 { fn default() -> F473 { F473::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub473 { pub e: E473, pub f: F473, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub473 { pub fn new(name: String) -> Sub473 { Sub473{e: Default::default(), f: F473::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type473 { pub sub: Sub473, pub e: E473, pub f: F473, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type473 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type473{sub: Default::default(), e: Default::default(), f: F473::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type473 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E474 { A, B, C }
impl Default for E474 { fn default() -> E474 { E474::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F474 { A, B, C }
impl Default for F474 { fn default() -> F474 { F474::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub474 { pub e: E474, pub f: F474, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub474 { pub fn new(name: String) -> Sub474 { Sub474{e: Default::default(), f: F474::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type474 { pub sub: Sub474, pub e: E474, pub f: F474, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type474 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type474{sub: Default::default(), e: Default::default(), f: F474::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type474 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E475 { A, B, C }
impl Default for E475 { fn default() -> E475 { E475::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F475 { A, B, C }
impl Default for F475 { fn default() -> F475 { F475::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub475 { pub e: E475, pub f: F475, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub475 { pub fn new(name: String) -> Sub475 { Sub475{e: Default::default(), f: F475::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type475 { pub sub: Sub475, pub e: E475, pub f: F475, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type475 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type475{sub: Default::default(), e: Default::default(), f: F475::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type475 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E476 { A, B, C }
impl Default for E476 { fn default() -> E476 { E476::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F476 { A, B, C }
impl Default for F476 { fn default() -> F476 { F476::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub476 { pub e: E476, pub f: F476, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub476 { pub fn new(name: String) -> Sub476 { Sub476{e: Default::default(), f: F476::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type476 { pub sub: Sub476, pub e: E476, pub f: F476, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type476 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type476{sub: Default::default(), e: Default::default(), f: F476::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type476 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E477 { A, B, C }
impl Default for E477 { fn default() -> E477 { E477::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F477 { A, B, C }
impl Default for F477 { fn default() -> F477 { F477::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub477 { pub e: E477, pub f: F477, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub477 { pub fn new(name: String) -> Sub477 { Sub477{e: Default::default(), f: F477::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type477 { pub sub: Sub477, pub e: E477, pub f: F477, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type477 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type477{sub: Default::default(), e: Default::default(), f: F477::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type477 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E478 { A, B, C }
impl Default for E478 { fn default() -> E478 { E478::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F478 { A, B, C }
impl Default for F478 { fn default() -> F478 { F478::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub478 { pub e: E478, pub f: F478, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub478 { pub fn new(name: String) -> Sub478 { Sub478{e: Default::default(), f: F478::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type478 { pub sub: Sub478, pub e: E478, pub f: F478, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type478 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type478{sub: Default::default(), e: Default::default(), f: F478::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type478 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E479 { A, B, C }
impl Default for E479 { fn default() -> E479 { E479::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F479 { A, B, C }
impl Default for F479 { fn default() -> F479 { F479::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub479 { pub e: E479, pub f: F479, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub479 { pub fn new(name: String) -> Sub479 { Sub479{e: Default::default(), f: F479::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type479 { pub sub: Sub479, pub e: E479, pub f: F479, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type479 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type479{sub: Default::default(), e: Default::default(), f: F479::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type479 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E480 { A, B, C }
impl Default for E480 { fn default() -> E480 { E480::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F480 { A, B, C }
impl Default for F480 { fn default() -> F480 { F480::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub480 { pub e: E480, pub f: F480, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub480 { pub fn new(name: String) -> Sub480 { Sub480{e: Default::default(), f: F480::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type480 { pub sub: Sub480, pub e: E480, pub f: F480, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type480 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type480{sub: Default::default(), e: Default::default(), f: F480::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type480 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E481 { A, B, C }
impl Default for E481 { fn default() -> E481 { E481::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F481 { A, B, C }
impl Default for F481 { fn default() -> F481 { F481::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub481 { pub e: E481, pub f: F481, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub481 { pub fn new(name: String) -> Sub481 { Sub481{e: Default::default(), f: F481::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type481 { pub sub: Sub481, pub e: E481, pub f: F481, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type481 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type481{sub: Default::default(), e: Default::default(), f: F481::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type481 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E482 { A, B, C }
impl Default for E482 { fn default() -> E482 { E482::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F482 { A, B, C }
impl Default for F482 { fn default() -> F482 { F482::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub482 { pub e: E482, pub f: F482, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub482 { pub fn new(name: String) -> Sub482 { Sub482{e: Default::default(), f: F482::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type482 { pub sub: Sub482, pub e: E482, pub f: F482, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type482 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type482{sub: Default::default(), e: Default::default(), f: F482::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type482 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E483 { A, B, C }
impl Default for E483 { fn default() -> E483 { E483::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F483 { A, B, C }
impl Default for F483 { fn default() -> F483 { F483::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub483 { pub e: E483, pub f: F483, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub483 { pub fn new(name: String) -> Sub483 { Sub483{e: Default::default(), f: F483::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type483 { pub sub: Sub483, pub e: E483, pub f: F483, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type483 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type483{sub: Default::default(), e: Default::default(), f: F483::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type483 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E484 { A, B, C }
impl Default for E484 { fn default() -> E484 { E484::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F484 { A, B, C }
impl Default for F484 { fn default() -> F484 { F484::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub484 { pub e: E484, pub f: F484, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub484 { pub fn new(name: String) -> Sub484 { Sub484{e: Default::default(), f: F484::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type484 { pub sub: Sub484, pub e: E484, pub f: F484, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type484 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type484{sub: Default::default(), e: Default::default(), f: F484::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type484 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E485 { A, B, C }
impl Default for E485 { fn default() -> E485 { E485::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F485 { A, B, C }
impl Default for F485 { fn default() -> F485 { F485::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub485 { pub e: E485, pub f: F485, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub485 { pub fn new(name: String) -> Sub485 { Sub485{e: Default::default(), f: F485::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type485 { pub sub: Sub485, pub e: E485, pub f: F485, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type485 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type485{sub: Default::default(), e: Default::default(), f: F485::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type485 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E486 { A, B, C }
impl Default for E486 { fn default() -> E486 { E486::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F486 { A, B, C }
impl Default for F486 { fn default() -> F486 { F486::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub486 { pub e: E486, pub f: F486, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub486 { pub fn new(name: String) -> Sub486 { Sub486{e: Default::default(), f: F486::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type486 { pub sub: Sub486, pub e: E486, pub f: F486, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type486 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type486{sub: Default::default(), e: Default::default(), f: F486::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type486 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E487 { A, B, C }
impl Default for E487 { fn default() -> E487 { E487::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F487 { A, B, C }
impl Default for F487 { fn default() -> F487 { F487::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub487 { pub e: E487, pub f: F487, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub487 { pub fn new(name: String) -> Sub487 { Sub487{e: Default::default(), f: F487::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type487 { pub sub: Sub487, pub e: E487, pub f: F487, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type487 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type487{sub: Default::default(), e: Default::default(), f: F487::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type487 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E488 { A, B, C }
impl Default for E488 { fn default() -> E488 { E488::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F488 { A, B, C }
impl Default for F488 { fn default() -> F488 { F488::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub488 { pub e: E488, pub f: F488, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub488 { pub fn new(name: String) -> Sub488 { Sub488{e: Default::default(), f: F488::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type488 { pub sub: Sub488, pub e: E488, pub f: F488, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type488 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type488{sub: Default::default(), e: Default::default(), f: F488::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type488 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E489 { A, B, C }
impl Default for E489 { fn default() -> E489 { E489::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F489 { A, B, C }
impl Default for F489 { fn default() -> F489 { F489::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub489 { pub e: E489, pub f: F489, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub489 { pub fn new(name: String) -> Sub489 { Sub489{e: Default::default(), f: F489::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type489 { pub sub: Sub489, pub e: E489, pub f: F489, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type489 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type489{sub: Default::default(), e: Default::default(), f: F489::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type489 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E490 { A, B, C }
impl Default for E490 { fn default() -> E490 { E490::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F490 { A, B, C }
impl Default for F490 { fn default() -> F490 { F490::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub490 { pub e: E490, pub f: F490, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub490 { pub fn new(name: String) -> Sub490 { Sub490{e: Default::default(), f: F490::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type490 { pub sub: Sub490, pub e: E490, pub f: F490, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type490 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type490{sub: Default::default(), e: Default::default(), f: F490::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type490 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E491 { A, B, C }
impl Default for E491 { fn default() -> E491 { E491::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F491 { A, B, C }
impl Default for F491 { fn default() -> F491 { F491::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub491 { pub e: E491, pub f: F491, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub491 { pub fn new(name: String) -> Sub491 { Sub491{e: Default::default(), f: F491::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type491 { pub sub: Sub491, pub e: E491, pub f: F491, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type491 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type491{sub: Default::default(), e: Default::default(), f: F491::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type491 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E492 { A, B, C }
impl Default for E492 { fn default() -> E492 { E492::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F492 { A, B, C }
impl Default for F492 { fn default() -> F492 { F492::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub492 { pub e: E492, pub f: F492, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub492 { pub fn new(name: String) -> Sub492 { Sub492{e: Default::default(), f: F492::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type492 { pub sub: Sub492, pub e: E492, pub f: F492, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type492 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type492{sub: Default::default(), e: Default::default(), f: F492::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type492 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E493 { A, B, C }
impl Default for E493 { fn default() -> E493 { E493::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F493 { A, B, C }
impl Default for F493 { fn default() -> F493 { F493::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub493 { pub e: E493, pub f: F493, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub493 { pub fn new(name: String) -> Sub493 { Sub493{e: Default::default(), f: F493::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type493 { pub sub: Sub493, pub e: E493, pub f: F493, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type493 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type493{sub: Default::default(), e: Default::default(), f: F493::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type493 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E494 { A, B, C }
impl Default for E494 { fn default() -> E494 { E494::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F494 { A, B, C }
impl Default for F494 { fn default() -> F494 { F494::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub494 { pub e: E494, pub f: F494, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub494 { pub fn new(name: String) -> Sub494 { Sub494{e: Default::default(), f: F494::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type494 { pub sub: Sub494, pub e: E494, pub f: F494, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type494 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type494{sub: Default::default(), e: Default::default(), f: F494::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type494 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E495 { A, B, C }
impl Default for E495 { fn default() -> E495 { E495::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F495 { A, B, C }
impl Default for F495 { fn default() -> F495 { F495::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub495 { pub e: E495, pub f: F495, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub495 { pub fn new(name: String) -> Sub495 { Sub495{e: Default::default(), f: F495::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type495 { pub sub: Sub495, pub e: E495, pub f: F495, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type495 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type495{sub: Default::default(), e: Default::default(), f: F495::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type495 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E496 { A, B, C }
impl Default for E496 { fn default() -> E496 { E496::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F496 { A, B, C }
impl Default for F496 { fn default() -> F496 { F496::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub496 { pub e: E496, pub f: F496, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub496 { pub fn new(name: String) -> Sub496 { Sub496{e: Default::default(), f: F496::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type496 { pub sub: Sub496, pub e: E496, pub f: F496, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type496 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type496{sub: Default::default(), e: Default::default(), f: F496::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type496 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E497 { A, B, C }
impl Default for E497 { fn default() -> E497 { E497::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F497 { A, B, C }
impl Default for F497 { fn default() -> F497 { F497::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub497 { pub e: E497, pub f: F497, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub497 { pub fn new(name: String) -> Sub497 { Sub497{e: Default::default(), f: F497::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type497 { pub sub: Sub497, pub e: E497, pub f: F497, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type497 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type497{sub: Default::default(), e: Default::default(), f: F497::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type497 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E498 { A, B, C }
impl Default for E498 { fn default() -> E498 { E498::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F498 { A, B, C }
impl Default for F498 { fn default() -> F498 { F498::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub498 { pub e: E498, pub f: F498, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub498 { pub fn new(name: String) -> Sub498 { Sub498{e: Default::default(), f: F498::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type498 { pub sub: Sub498, pub e: E498, pub f: F498, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type498 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type498{sub: Default::default(), e: Default::default(), f: F498::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type498 { fn name(&self) -> &str { &self.name } }

#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum E499 { A, B, C }
impl Default for E499 { fn default() -> E499 { E499::A } }
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Debug,Serialize,Deserialize)]
pub enum F499 { A, B, C }
impl Default for F499 { fn default() -> F499 { F499::A } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Sub499 { pub e: E499, pub f: F499, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Sub499 { pub fn new(name: String) -> Sub499 { Sub499{e: Default::default(), f: F499::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()} } }
#[derive(Clone,PartialEq,Eq,Debug,Default,Serialize,Deserialize)]
pub struct Type499 { pub sub: Sub499, pub e: E499, pub f: F499, pub name: String, pub other: Option<String>, pub s: Option<String>, pub v: Vec<i64>}
impl Type499 { pub fn new(name: String) -> std::boxed::Box<Named> { std::boxed::Box::new(Type499{sub: Default::default(), e: Default::default(), f: F499::B, name: name, other: None, s: Some("foo".into()), v: Vec::new()}) } }
impl Named for Type499 { fn name(&self) -> &str { &self.name } }

