//mod utils;

use wasm_bindgen::prelude::*;
use regex::Regex;
use std::collections::HashMap;

mod css;
use css::core::get_acss;
use css::core::get_breakpoints;
use css::custom_class::get_init_css;
use std::borrow::Borrow;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const PSEUDO_CLASSES: [&str;35] =  [ ":a",":c",":d",":di",":e",":en",":fi",":fc",":fot",":fs",":f",":fw",":fv",
":h", ":ind",":ir",":inv",":lc",":lot",":l",":li",":oc",":oot",":o",":oor",":ps", ":ro",":rw",":req",
":r",":rt", ":s", ":t",":va", ":vi"];
const PSEUDO_CLASSES_VEB: [&str;35] =  [
    ":active",":checked",":default", ":disabled", ":empty",":enabled", ":first",":first-child",
    ":first-of-type",":fullscreen",":focus",  ":focus-within", ":focus-visible", ":hover", ":indeterminate",
    ":in-range", ":invalid", ":last-child", ":last-of-type",":left",":link", ":only-child", ":only-of-type",
    ":optional",":out-of-range",":placeholder-shown", ":read-only", ":read-write",":required", ":right",
    ":root", ":scope", ":target", ":valid", ":visited"  ];  

const PSEUDO_ELEMENTS: [&str;5] = ["::b","::a","::fl","::fli","::ph"];
const PSEUDO_ELEMENTS_VEB: [&str;5] = ["::before","::after","::first-letter","::first-line","::placeholder"];

const COMBINATOR_ELEMENTS: [&str;4] = ["_","+",">","~"];
const COMBINATOR_ELEMENTS_VEB: [&str;4] = [" "," + "," > "," ~ "];

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn add_init_css()->String{
    get_init_css()
}
#[wasm_bindgen]
pub fn atomic_css_compile_from_html(html:String)->String {
    let regex = Regex::new(r#"class="([^"]+)""#).unwrap();
    let mut ret:String = "".to_string();
    for cap in regex.captures_iter(&html) {
        ret = ret + &cap[1] + " ";
    }
    atomic_css_compile(ret)
}

#[wasm_bindgen]
pub fn atomic_css_compile(classes:String)->String {
    css_hashmap_to_string(atomic_css_classes(css_dedup_classes(classes)))
}
fn css_hashmap_to_string(all_css:HashMap<String, Vec<String>>)->String {
    let css_breakpoints = get_breakpoints();
    let mut css = "".to_owned();
    for (pb, v) in all_css.iter() {
        if pb.to_owned() == "all".to_owned(){
            css += v.join("").as_str();
        }else{
            if let Some(queryvar) = css_breakpoints.get(&pb.as_str()){
                css = css + queryvar + r#"{"#;
                css += v.join("").as_str();
                css = css + r#"}"#;
            }
        }
    }
    css
}

fn atomic_css_classes(classes:String)->HashMap<String, Vec<String>> {
    let class_pattern:String = r#"([a-z]*)(["#.to_owned()+PSEUDO_CLASSES.join("|").as_str()+r#"]*)(["#+COMBINATOR_ELEMENTS.join("|").as_str()+r#"]?)([A-Z][a-z]*)\(([^)]*)\)([!]?)(["#+PSEUDO_CLASSES.join("|").as_str()+r#"]*)(["#+PSEUDO_ELEMENTS.join("|").as_str()+r#"]*)(-?-?([a-z]*))"#;
    let class_regex = Regex::new(class_pattern.as_str()).unwrap();
    
    let mut all_css:HashMap<String, Vec<String>> = HashMap::new();
    
    for capture in class_regex.captures_iter(&classes) {
        if capture.len()==11 {
       
            let result:String =  simple_class_verbose(&capture[1])+pseudo_class_verbose(&capture[2])+combination_element_verbose(&capture[3])+ 
            r#"."# + &capture[4]+r#"\("#+ add_css_slash(&capture[5]).as_str()+r#"\)"# +pseudo_class_element(&capture[7]).as_str()+ filter_important_param(&capture[6]).as_str() +&capture[9]+ r#"{"# +get_css_content(&capture[4],&capture[5],&capture[6]).as_str() +r#"}"#;
            
            let mut bp:String = "all".to_owned();
            if &capture[10]!=""{
                bp = capture[10].to_string();
            }
            if let Some(val) = all_css.get_mut(&bp) {
                val.push(result);
            }else{
                all_css.insert(bp,vec![result]);
            }
        }
        
    }
    all_css 
}
fn simple_class_verbose(pc:&str)->String{
    if pc!="" {
        r#"."#.to_string()+pc
    }else{
        "".to_string()
    }
}
fn filter_important_param(pc:&str)->String{
    if pc=="" {
        "".to_string()
    }else{
        r#"\!"#.to_string()
    }
}
fn pseudo_class_verbose(pc:&str)->&str{    
    let index_element = PSEUDO_CLASSES
        .iter()
        .position(|&x| x == pc);
        if let Some(ind) = index_element{
            PSEUDO_CLASSES_VEB[ind]
        }else{
            ""
        }
}
fn pseudo_class_element(pc:&str)->String{
    let elem_regex = Regex::new(r#"::[a-z]+"#).unwrap();
    let pseudo_regex = Regex::new(r#":[a-z]+"#).unwrap();
    
    let mut ret1:String = "".to_string();
    let mut ret2:String = "".to_string();
    for cap in elem_regex.captures_iter(pc) {
                ret1 = pseudo_element_verbose(&cap[0]).to_string();
            }
    for cap in pseudo_regex.captures_iter(pc) {
                ret2 = pseudo_class_verbose(&cap[0]).to_string();
            }        
            ret1+ret2.as_str()
}
fn pseudo_element_verbose(pc:&str)->&str{    
    let index_element = PSEUDO_ELEMENTS
        .iter()
        .position(|&x| x == pc);
        if let Some(ind) = index_element{
            PSEUDO_ELEMENTS_VEB[ind] 
        }else{
            "" 
        }
}
fn combination_element_verbose(pc:&str)->&str{
    let index_element = COMBINATOR_ELEMENTS
    .iter()
    .position(|&x| x == pc);
    if let Some(ind) = index_element{
        COMBINATOR_ELEMENTS_VEB[ind]
    }else{
        ""
    }
}
//Animdir\(<([^>)]*)>,?<?([^>)]*)?>?,?<?([^>)]*)?>?,?<?([^>)]*)?>?\)
//a:([^,]+)
fn get_css_content(class:&str,param:&str,important:&str)->String{
    let ascss_classes = get_acss();
    let class_pattern:String = (class.to_string()+r#"\(<([^>)]*)>,?<?([^>)]*)?>?,?<?([^>)]*)?>?,?<?([^>)]*)?>?\)\{([^}]*)\}"#).to_string();
    let params: Vec<&str> = param.split(",").collect();
    let class_regex = Regex::new(class_pattern.as_str()).unwrap();
    let mut ret:String = "".to_string();

    for cap in class_regex.captures_iter(&ascss_classes) {
        ret = cap[5].to_string();
        if params.len() == 1{
            let param_css_ver = get_param_verbose(&param,&cap[1]);
            let param_css = filter_css_param(&param_css_ver);
            ret = ret.replace("$0",param_css.as_str());
        }else if params.len() > 1{
            for i in 0..params.len() {
                let param_css_ver = get_param_verbose(&params[i],&cap[i+1]);
                let param_css = filter_css_param(&param_css_ver);
                ret = ret.replace(("$".to_owned()+(i).to_string().as_str()).as_str(),param_css.as_str() );
            }
        }
    }
    if important=="!"{ret = css_filter_important(ret);}
    ret
}

fn get_param_verbose(param:&str,data:&str) -> String{
    let param_pattern:String = param.to_string() + r#":([^,]+)"#;
    let param_regex = Regex::new(param_pattern.as_str()).unwrap();
    let mut ret:String = "".to_string();
    if param_regex.is_match(data) {
        for cap in param_regex.captures_iter(data) {
            ret = cap[1].to_string();
        }
    }else{
        ret = param.to_string();
    } 
    ret
}
fn add_css_slash(param:&str)-> String{
    let rg = Regex::new(r#"[.|,|#|?|+|*|/|\[|\]|%]{1}"#).unwrap();
    let mut ret: String = "".to_string();
    ret.push_str(rg.replace_all(param, "\\$0").borrow());
    ret
}
// 1/2  ^\d{1,2}/\d{1,2}$
// [100%-20px]  \[([^]]*)\] 
fn filter_css_param(param:&str)->String{
    let regex_var = Regex::new(r#"--\S+"#).unwrap();
    let regex_fraction = Regex::new(r#"^\d{1,2}/\d{1,2}$"#).unwrap();
    let regex_calc = Regex::new(r#"\[([^]]*)\]"#).unwrap();
    if regex_var.is_match(param){
        (r#"var("#.to_owned()+param+r#")"#).to_string()
    }else if regex_fraction.is_match(param){
        (r#"calc(100%*"#.to_owned()+param+r#")"#).to_string()
    }else if regex_calc.is_match(param){
        param.to_string().replace("[","cacl(").replace("]",")")
    }else{
        param.to_string()
    }
}
 
fn css_filter_important(css:String)->String{
    css.replace(";","!important;")
}
fn css_dedup_classes(classes:String)->String {
     let mut params: Vec<&str> = classes.split(" ").collect();
     params.sort();
     params.dedup();
     params.join(" ")
}