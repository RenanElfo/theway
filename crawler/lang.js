const BASE_PATH = 'https://escriva.org';

export const languages = [
  'es',
  'bg',
  'ca',
  'cs',
  'de',
  'en',
  'fi',
  'fr',
  'hr',
  'it',
  'ja',
  'ko',
  'lt',
  'lv',
  'hu',
  'nl',
  'pl',
  'pt-br',
  'pt-pt',
  'ro',
  'ru',
  'sk',
  'sl',
  'sv',
  'zh-hans',
  'zh-hant',
]

export function getPathWithLanguage(lang) {
  return BASE_PATH + '/' + lang + '/camino/';
}

