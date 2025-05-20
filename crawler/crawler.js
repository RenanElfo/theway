import FileSystem from 'fs';

import { JSDOM } from 'jsdom';

import { languages, getPathWithLanguage } from './lang.js';

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
  const languages = LANGUAGES; // comment/uncomment to run this for just pt-br
  const theWay = [];

  let lang;
  for (let langIndex = 0; langIndex < languages.length; langIndex++) {
    lang = languages[langIndex];
    console.log(`Crawling for language ${lang}`);
    for (let i = 0; i < NUMBER_OF_POINTS; i++) {
      console.log(`Crawling for point ${i+1}`);
      sleep(CRAWL_DELAY_MS);
      const r = await fetch(getPathWithLanguage(lang) + (i+1).toString())
        .then((response) => {
          if (!response.ok) {
            throw new Error(`HTTP error: ${response.status}`);
          }
          return response.text();
        })
        .then((text) => {
          const dom = new JSDOM(text);
          const document = dom.window.document;

          // Crawl for point
          const contentAsHtml = document.getElementById('contenido');
          const paragraphsAsHtml = contentAsHtml.getElementsByTagName('p');
          const contentAsHtmlArray = Array.prototype.slice.call(paragraphsAsHtml);
          const point = contentAsHtmlArray
            .map((p) => p.innerHTML)
            .slice(0, -1)
            .join('\n\n');

          // Crawl for subject
          const parentDiv = document.getElementsByClassName('pre-destacado')[0];
          const subject = parentDiv.getElementsByTagName('a')[0].innerHTML;

          return { subject, point };
        });
      console.log(r);
      theWay.push(r);
    }
    FileSystem.writeFile(
      'caminho.json',
      JSON.stringify(theWay, undefined, 2) + '\n\n',
      (error) => {
        if (error) throw error;
    });
  }
}

getHtmlAsText();

