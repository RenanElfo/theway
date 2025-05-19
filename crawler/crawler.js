import { JSDOM } from 'jsdom';

import { languages, getPathWithLanguage } from './lang.js';

function sleep(delayMs) {
  const start = Date.now();
  while (Date.now() < start + delayMs);
}

async function getHtmlAsText() {
  languages.forEach((lang) => {
    console.log(`Crawling for language ${lang}`);
    sleep(1_000);
    return await fetch(getPathWithLanguage('pt-br') + '1')
      .then((response) => {
        if (!response.ok) {
          throw new Error(`HTTP error: ${response.status}`);
        }
        return response.text();
      })
      .then((text) => {
        const dom = new JSDOM(text);
        const document = dom.window.document;
        const contentAsHtml = document.getElementById('contenido');
        const paragraphsAsHtml = contentAsHtml.getElementsByTagName('p');
        const contentAsHtmlArray = Array.prototype.slice.call(paragraphsAsHtml);
        const point = contentAsHtmlArray
          .map((p) => p.innerHTML)
          .slice(0, -1)
          .join('\n\n');
        console.log(point);
      });
    }
  );
}

console.log('html', getHtmlAsText());

