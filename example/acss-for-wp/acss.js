import {
    default as wasm,
    add_init_css,
    atomic_css_compile,
    atomic_css_compile_from_html,
  } from "./wasm/acss.js";

  document.addEventListener("DOMContentLoaded", function () {
   
    wasm().then((module) => {
      add_css_to_body(add_init_css());

      const body = document.getElementsByTagName("BODY")[0];

      let compiledcss = atomic_css_compile_from_html(body.innerHTML);

      add_css_to_body(compiledcss);

      window.atomic_css_compile = atomic_css_compile;
      window.atomic_css_compile_from_html = atomic_css_compile_from_html;
    });
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
   