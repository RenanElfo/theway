const BASE_PATH = 'https://escriva.org';

export const languages = [
	{ locale: 'es', bookName: 'Camino' },
	{ locale: 'bg', bookName: 'Път' },
	{ locale: 'ca', bookName: 'Camí' },
	{ locale: 'cs', bookName: 'Cesta' },
	{ locale: 'de', bookName: 'Der Weg' },
	{ locale: 'en', bookName: 'The Way' },
	{ locale: 'fi', bookName: 'Tie' },
	{ locale: 'fr', bookName: 'Chemin' },
	{ locale: 'hr', bookName: 'Put' },
	{ locale: 'it', bookName: 'Cammino' },
	{ locale: 'ja', bookName: '道' },
	{ locale: 'ko', bookName: '길' },
	{ locale: 'lt', bookName: 'Kelias' },
	{ locale: 'lv', bookName: 'Ceļš' },
	{ locale: 'hu', bookName: 'Út' },
	{ locale: 'nl', bookName: 'De Weg' },
	{ locale: 'pl', bookName: 'Droga' },
	{ locale: 'pt-br', bookName: 'Caminho' },
	{ locale: 'pt-pt', bookName: 'Caminho' },
	{ locale: 'ro', bookName: 'Drum' },
	{ locale: 'ru', bookName: 'Путь' },
	{ locale: 'sk', bookName: 'Cesta' },
	{ locale: 'sl', bookName: 'Pot' },
	{ locale: 'sv', bookName: 'Vägen' },
	{ locale: 'zh-hans', bookName: '道路' },
	{ locale: 'zh-hant', bookName: '道路' },
]

export function getPathWithLanguage(lang) {
  return BASE_PATH + '/' + lang + '/camino/';
}

