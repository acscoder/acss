import {
    default as wasm,
    add_init_css,
    atomic_css_compile,
    atomic_css_compile_from_html,
  } from "./wasm/acss.js";

    wasm().then((module) => {
      add_css_to_body(add_init_css());

      const body = document.getElementsByTagName("BODY")[0];

      let compiledcss = atomic_css_compile_from_html(body.innerHTML);

      add_css_to_body(compiledcss);

      window.atomic_css_compile = atomic_css_compile;
      window.atomic_css_compile_from_html = atomic_css_compile_from_html;
    });
  
  function add_css_to_body(css) {
    var head = document.head || document.getElementsByTagName("head")[0],
      style = document.createElement("style");
    head.appendChild(style);
    style.type = "text/css";
    if (style.styleSheet) {
      // This is required for IE8 and below.
      style.styleSheet.cssText = css;
    } else {
      style.appendChild(document.createTextNode(css));
    }
  }
   
  var editor_changed = false;
  var all_classes = "";
  document.getElementById("editor").addEventListener('DOMSubtreeModified', function () {
    editor_changed = true;
  }, false);
setInterval(function() {
  if(editor_changed){
    var classes = '';
    document.getElementById("editor").querySelectorAll("*").forEach(function(elem){
      classes += " "+elem.classList;
    })
    if(all_classes!=classes){
      all_classes = classes;
      add_css_to_body(atomic_css_compile(all_classes));
    }
    editor_changed = false;
  }
},500);
  

