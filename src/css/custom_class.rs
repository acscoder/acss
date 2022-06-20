pub const ASCSS_CLASSES_CUSTOM : &str = r#"
GridCols(<>){grid-template-columns: repeat($0, minmax(0, 1fr));}
GridColsNone(<>){grid-template-columns:none;}
ColStart(<:auto>){grid-column-start: $0;}
ColEnd(<:auto>){grid-column-end: $0;}
ColSpan(<>){grid-column: span $0 / span $0;}
ColAuto(<>){grid-column: auto;}
ColSpanFull(<>){grid-column: 1 / -1;}

Gap(<:var(--gap)>){gap: $0;}
GapX(<:var(--gapx)>){column-gap: $0;}
GapY(<:var(--gapy)>){row-gap: $0;}
GridAutoRows(<a:auto,min:min-content,max:max-content,fr:minmax(0,1fr)>){grid-auto-columns:$0;}
GridAutoCols(<a:auto,min:min-content,max:max-content,fr:minmax(0,1fr)>){grid-auto-columns:$0;}
GridAutoFlow(<r:row,c:column,d:dense,rd:row dense,cd:column dense>){grid-auto-flow:$0;}

GridRows(<>){grid-template-rows: repeat($0, minmax(0, 1fr));}
GridRowsNone(<>){grid-template-rows:none;}
RowStart(<:auto>){grid-row-start: $0;}
RowEnd(<:auto>){grid-row-end: $0;}
RowSpan(<>){grid-row: span $0 / span $0;}
RowAuto(<>){grid-row: auto;}
RowSpanFull(<>){grid-row: 1 / -1;}
PlaceContent(<c:center,s:start,e:end,b:space-between,a:space-around,e:space-evenly,s:stretch>){place-content: $0;}
PlaceItems(<c:center,s:start,e:end,s:stretch>){place-items: $0;}
PlaceSelf(<a:auto,c:center,s:start,e:end,s:stretch>){place-self: $0;}
"#;

const CSS_RESET : &str = r#"
html{line-height:1.15;-webkit-text-size-adjust:100%}body{margin:0}main{display:block}hr{box-sizing:content-box;height:0;overflow:visible}pre{font-family:monospace,monospace;font-size:1em}a{background-color:transparent}abbr[title]{border-bottom:none;text-decoration:underline;text-decoration:underline dotted}b,strong{font-weight:bolder}code,kbd,samp{font-family:monospace,monospace;font-size:1em}small{font-size:80%}sub,sup{font-size:75%;line-height:0;position:relative;vertical-align:baseline}sub{bottom:-.25em}sup{top:-.5em}img{border-style:none}button,input,optgroup,select,textarea{font-family:inherit;font-size:100%;line-height:1.15;margin:0}button,input{overflow:visible}button,select{text-transform:none}button,[type="button"],[type="reset"],[type="submit"]{-webkit-appearance:button}button::-moz-focus-inner,[type="button"]::-moz-focus-inner,[type="reset"]::-moz-focus-inner,[type="submit"]::-moz-focus-inner{border-style:none;padding:0}button:-moz-focusring,[type="button"]:-moz-focusring,[type="reset"]:-moz-focusring,[type="submit"]:-moz-focusring{outline:1px dotted ButtonText}fieldset{padding:.35em .75em .625em}legend{box-sizing:border-box;color:inherit;display:table;max-width:100%;padding:0;white-space:normal}progress{vertical-align:baseline}textarea{overflow:auto}[type="checkbox"],[type="radio"]{box-sizing:border-box;padding:0}[type="number"]::-webkit-inner-spin-button,[type="number"]::-webkit-outer-spin-button{height:auto}[type="search"]{-webkit-appearance:textfield;outline-offset:-2px}[type="search"]::-webkit-search-decoration{-webkit-appearance:none}::-webkit-file-upload-button{-webkit-appearance:button;font:inherit}details{display:block}summary{display:list-item}template{display:none}[hidden]{display:none}
"#;
const CSS_CUSTOM  : &str = r#""#;
pub fn get_init_css()->String{
    CSS_RESET.to_string() + CSS_CUSTOM
}