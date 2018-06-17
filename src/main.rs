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

    let r = translate(&args[0]);
    match r {
        Ok(r) => println!("{}", r),
        _ => println!("Not translated")
    }
}

/**
 * translate specified word from english to japanese.
 */
fn translate(word: &str) -> Result<String, Box<std::error::Error>> {
    let s = search_word(word);
    let word_id = parse(&(s.unwrap()));

    // translate word.
    let url = format!("http://public.dejizo.jp/NetDicV09.asmx/GetDicItemLite?Dic=EJdict&Item={}&Loc=&Prof=XHTML", word_id);
    let mut r = reqwest::get(&url)?;
    let mut b = String::new();
    r.read_to_string(&mut b)?;
    let translated = parse(&b);

    Ok(translated.to_string())
}

/**
 * seach word section.
 */
fn search_word(word: &str) -> Result<String, Box<std::error::Error>> {
    let url = format!("http://public.dejizo.jp/NetDicV09.asmx/SearchDicItemLite?Dic=EJdict&Word={}&Scope=HEADWORD&Match=STARTWITH&Merge=AND&Prof=XHTML&PageSize=1&PageIndex=0", word);
    let mut r = reqwest::get(&url)?;

    // set getting xml.
    let mut b = String::new();
    r.read_to_string(&mut b)?;

    Ok(b)
}

/**
 * parse xml.
 */
fn parse(str: &str) -> String {
    let mut reader = Reader::from_str(str);
    reader.trim_text(true);
    let mut buf = Vec::new();

    let mut pre_tag = "".to_string();
    let mut content: String = "".to_string();
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let tag = String::from_utf8(e.name().to_vec()).unwrap();
                match &*tag {
                    // search item id.
                    "ItemID" =>{
                        content = reader.read_text(e.name(), &mut Vec::new()).unwrap();
                        break
                    },
                    // search translated content.
                    "Body" => pre_tag = "Body".to_string(),
                    _ => (),
                }
            },
            Ok(Event::Text(e)) => {
                if pre_tag == "Body" {
                    content = e.unescape_and_decode(&reader).unwrap();
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
