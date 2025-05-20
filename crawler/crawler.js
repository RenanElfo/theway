import { JSDOM } from 'jsdom';

import { getPathWithLanguage } from './lang.js';

const NUMBER_OF_POINTS = 999;

// Necessary delay to respect Crawl-delay specified in escriva.org/robots.txt
const CRAWL_DELAY_MS = 10_000;
// Using only pt-br for now.
const LANGUAGES = ['pt-br']

function sleep(delayMs) {
  const start = Date.now();
  while (Date.now() < start + delayMs);
}

async function getHtmlAsText() {
  const languages = LANGUAGES;
  languages.forEach(async (lang) => {
    console.log(`Crawling for language ${lang}`);
    for (let i = 0; i < 2; i++) {
      console.log(`Crawling for point ${i}`);
      sleep(CRAWL_DELAY_MS);
      const r = await fetch(getPathWithLanguage(lang) + (i+1).toString())
        .then((response) => {
          if (!response.ok) {
            throw new Error(`HTTP error: ${response.status},
              ${getPathWithLanguage(lang) + i.toString()}
            `);
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
          return point;
        });
      console.log(r);
    }
  });
}

getHtmlAsText();

