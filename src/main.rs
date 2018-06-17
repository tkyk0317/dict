extern crate reqwest;
extern crate quick_xml;

use quick_xml::Reader;
use quick_xml::events::Event;
use std::io::Read;
use std::string::String;

fn main() {
    let mut args = Vec::new();
    for arg in std::env::args().skip(1) {
        args.push(arg);
    }

    if args.len() == 0 {
        println!("must argment");
        return;
    }

    let r = translate(&args[0]);
    match r {
        Ok(r) => println!("{}", r),
        _ => println!("translate error")
    }
}

/**
 * translate specified word from english to japanese.
 */
fn translate(word: &String) -> Result<String, String> {
    _translate(&_search_id(&word)?)
}

/**
 * seach word id.
 */
fn _search_id(word: &String) -> Result<String, String> {
    let url = format!("http://public.dejizo.jp/NetDicV09.asmx/SearchDicItemLite?Dic=EJdict&Word={}&Scope=HEADWORD&Match=STARTWITH&Merge=AND&Prof=XHTML&PageSize=1&PageIndex=0", word);
    let mut r =
        match reqwest::get(&url) {
            Ok(u) => u,
            Err(_) => return Err("reqwest::get is error in search_word".to_string())
        };

    // set getting xml.
    let mut b = String::new();
    match r.read_to_string(&mut b) {
        Ok(_) => {},
        Err(_) => return Err("read_to_string is error in search_word".to_string())
    };

    // parse item-id.
    parse(&b).ok_or("parse is error".to_string())
}

/**
 * process translate.
 */
fn _translate(word_id: &String) -> Result<String, String> {
    let url = format!("http://public.dejizo.jp/NetDicV09.asmx/GetDicItemLite?Dic=EJdict&Item={}&Loc=&Prof=XHTML", word_id);
    let mut r =
        match reqwest::get(&url) {
            Ok(u) => u,
            Err(_) => return Err("reqwest::get is error in _translate".to_string())
        };

    let mut b = String::new();
    match r.read_to_string(&mut b) {
        Ok(_) => {},
        Err(_) => return Err("read_to_string is error in _translate".to_string())
    }

    // parse item-id.
    parse(&b).ok_or("parse is error".to_string())
}

/**
 * parse xml.
 */
fn parse(str: &str) -> Option<String> {
    let mut reader = Reader::from_str(str);
    reader.trim_text(true);
    let mut buf = Vec::new();

    let mut pre_tag = "".to_string();
    let mut content = None;
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let tag = String::from_utf8(e.name().to_vec()).unwrap();
                match &*tag {
                    // search item id.
                    "ItemID" =>{
                        content = Some(reader.read_text(e.name(), &mut Vec::new()).unwrap());
                        break
                    },
                    // search translated content.
                    "Body" => pre_tag = "Body".to_string(),
                    _ => (),
                }
            },
            Ok(Event::Text(e)) => {
                if pre_tag == "Body" {
                    content = Some(e.unescape_and_decode(&reader).unwrap());
                    break
                }
            },
            Ok(Event::Eof) => break,
            _ => (),
        }
        buf.clear();
    }
    content
}

