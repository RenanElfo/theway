const jsdom = require("jsdom");
const { JSDOM } = jsdom;

async function getHtmlAsText() {
  return await fetch('https://escriva.org/en/camino/1/')
    .then((response) => {
      if (!response.ok) {
        throw new Error(`HTTP error: ${response.status}`);
      }
      return response.text();
    })
    .then((text) => {
      console.log('text', text);
      htmlAsText = text
    });
}

console.log('html', getHtmlAsText());

