// ==UserScript==
// @name         Input downloader
// @namespace    http://tampermonkey.net/
// @version      2023-12-09
// @description  try to take over the world!
// @author       You
// @match        https://adventofcode.com/*
// @icon         https://www.google.com/s2/favicons?sz=64&domain=adventofcode.com
// @run-at document-end
// ==/UserScript==

(function() {
    'use strict';

    const btn = document.createElement('button');

    Object.assign(btn.style, {
        position : 'absolute',
        top: '50px',
        right: '50px',
        background: 'none',
        border: 'none',
    });
    btn.innerHTML=`<svg xmlns="http://www.w3.org/2000/svg" height="16" width="16" viewBox="0 0 512 512"><!--!Font Awesome Free 6.5.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2023 Fonticons, Inc.--><path style="fill:white" d="M288 32c0-17.7-14.3-32-32-32s-32 14.3-32 32V274.7l-73.4-73.4c-12.5-12.5-32.8-12.5-45.3 0s-12.5 32.8 0 45.3l128 128c12.5 12.5 32.8 12.5 45.3 0l128-128c12.5-12.5 12.5-32.8 0-45.3s-32.8-12.5-45.3 0L288 274.7V32zM64 352c-35.3 0-64 28.7-64 64v32c0 35.3 28.7 64 64 64H448c35.3 0 64-28.7 64-64V416c0-35.3-28.7-64-64-64H346.5l-45.3 45.3c-25 25-65.5 25-90.5 0L165.5 352H64zm368 56a24 24 0 1 1 0 48 24 24 0 1 1 0-48z"/></svg>`

    function download(data) {
      fetch('http://localhost:8080?' + new URLSearchParams({
        path: document.URL,
      }), {
        method: 'POST',
        body: data,
      })
    }

    if (document.URL.endsWith('input')) {
        btn.addEventListener('click', ()=>{
          download(document.getElementsByTagName('pre')[0].innerText)
        });
        document.body.appendChild(btn);
    } else {
        document.querySelectorAll('pre > code').forEach(function(e) {
            const testInputBtn = btn.cloneNode(true);
            Object.assign(testInputBtn.style, {
                top: '10px',
                right: '0px',
            });
            testInputBtn.addEventListener('click', ()=>{
              download(e.innerText)
            });
            e.style.position = 'relative';
            e.appendChild(testInputBtn);
        })
    }
})();
