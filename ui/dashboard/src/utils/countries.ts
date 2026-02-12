/**
 * ISO 3166-1 alpha-2 code to display name (for Top Countries chart/list).
 * Pipeline returns ISO codes; we show full names in the UI.
 */
const ISO_TO_DISPLAY_NAME: Record<string, string> = {
  ad: 'Andorra', ae: 'United Arab Emirates', af: 'Afghanistan', al: 'Albania', am: 'Armenia', ao: 'Angola',
  aq: 'Antarctica', ar: 'Argentina', at: 'Austria', au: 'Australia', az: 'Azerbaijan', ba: 'Bosnia and Herzegovina',
  bd: 'Bangladesh', be: 'Belgium', bf: 'Burkina Faso', bg: 'Bulgaria', bh: 'Bhutan', bi: 'Burundi', bj: 'Benin',
  bn: 'Brunei', bo: 'Bolivia', br: 'Brazil', bs: 'Bahamas', bt: 'Bhutan', bw: 'Botswana', by: 'Belarus',
  bz: 'Belize', ca: 'Canada', cd: 'Democratic Republic of the Congo', cf: 'Central African Republic', cg: 'Congo',
  ch: 'Switzerland', ci: "Côte d'Ivoire", cl: 'Chile', cm: 'Cameroon', cn: 'China', co: 'Colombia', cr: 'Costa Rica',
  cu: 'Cuba', cz: 'Czechia', de: 'Germany', dj: 'Djibouti', dk: 'Denmark', do: 'Dominican Republic', dz: 'Algeria',
  ec: 'Ecuador', ee: 'Estonia', eg: 'Egypt', eh: 'Western Sahara', er: 'Eritrea', es: 'Spain', et: 'Ethiopia',
  fi: 'Finland', fj: 'Fiji', fr: 'France', ga: 'Gabon', gb: 'United Kingdom', ge: 'Georgia', gh: 'Ghana',
  gm: 'Gambia', gn: 'Guinea', gq: 'Equatorial Guinea', gr: 'Greece', gt: 'Guatemala', gw: 'Guinea-Bissau',
  gy: 'Guyana', hn: 'Honduras', hr: 'Croatia', ht: 'Haiti', hu: 'Hungary', id: 'Indonesia', ie: 'Ireland',
  il: 'Israel', in: 'India', iq: 'Iraq', ir: 'Iran', is: 'Iceland', it: 'Italy', jm: 'Jamaica', jo: 'Jordan',
  jp: 'Japan', ke: 'Kenya', kg: 'Kyrgyzstan', kh: 'Cambodia', kp: 'North Korea', kr: 'South Korea',
  kw: 'Kuwait', kz: 'Kazakhstan', la: 'Laos', lb: 'Lebanon', lk: 'Sri Lanka', lr: 'Liberia', ls: 'Lesotho',
  lt: 'Lithuania', lu: 'Luxembourg', lv: 'Latvia', ly: 'Libya', ma: 'Morocco', md: 'Moldova', me: 'Montenegro',
  mg: 'Madagascar', mk: 'North Macedonia', ml: 'Mali', mm: 'Myanmar', mn: 'Mongolia', mr: 'Mauritania', mt: 'Malta',
  mw: 'Malawi', mx: 'Mexico', my: 'Malaysia', mz: 'Mozambique', na: 'Namibia', ne: 'Niger', ng: 'Nigeria',
  ni: 'Nicaragua', nl: 'Netherlands', no: 'Norway', np: 'Nepal', nz: 'New Zealand', om: 'Oman', pa: 'Panama',
  pe: 'Peru', pg: 'Papua New Guinea', ph: 'Philippines', pk: 'Pakistan', pl: 'Poland', pt: 'Portugal',
  pr: 'Puerto Rico', ps: 'Palestine', py: 'Paraguay', qa: 'Qatar', ro: 'Romania', rs: 'Serbia', ru: 'Russia',
  rw: 'Rwanda', sa: 'Saudi Arabia', sb: 'Solomon Islands', sd: 'Sudan', se: 'Sweden', si: 'Slovenia', sk: 'Slovakia',
  sl: 'Sierra Leone', sn: 'Senegal', so: 'Somalia', sr: 'Suriname', ss: 'South Sudan', sv: 'El Salvador',
  sy: 'Syria', sz: 'Eswatini', td: 'Chad', tg: 'Togo', th: 'Thailand', tj: 'Tajikistan', tl: 'Timor-Leste',
  tm: 'Turkmenistan', tn: 'Tunisia', tr: 'Turkey', tt: 'Trinidad and Tobago', tz: 'Tanzania', ua: 'Ukraine',
  ug: 'Uganda', us: 'United States', uy: 'Uruguay', uz: 'Uzbekistan', ve: 'Venezuela', vn: 'Vietnam',
  vu: 'Vanuatu', ye: 'Yemen', za: 'South Africa', zm: 'Zambia', zw: 'Zimbabwe',
};

/**
 * Returns display name for a country. If the value is a 2-letter ISO code, returns the full name; otherwise returns the input.
 */
export function getCountryDisplayName(isoOrName: string): string {
  if (!isoOrName || typeof isoOrName !== 'string') return isoOrName || '';
  const trimmed = isoOrName.trim();
  if (trimmed.length === 2) {
    return ISO_TO_DISPLAY_NAME[trimmed.toLowerCase()] ?? trimmed.toUpperCase();
  }
  return trimmed;
}

// --- Topology keys (world-110m / Natural Earth) for map matching ---

/** Name variations to topology key (lowercase). Used by WorldMap. */
const API_TO_TOPOLOGY: Record<string, string> = {
  'united states': 'united states of america', 'usa': 'united states of america', 'us': 'united states of america',
  'russian federation': 'russia', 'korea, republic of': 'south korea', "korea, democratic people's republic of": 'north korea',
  'democratic republic of the congo': 'dem. rep. congo', 'democratic republic of congo': 'dem. rep. congo',
  'dr congo': 'dem. rep. congo', 'congo, democratic republic of the': 'dem. rep. congo', 'congo-kinshasa': 'dem. rep. congo',
  'republic of the congo': 'congo', 'congo-brazzaville': 'congo', 'ivory coast': "côte d'ivoire", "cote d'ivoire": "côte d'ivoire",
  'dominican republic': 'dominican rep.', 'central african republic': 'central african rep.', 'equatorial guinea': 'eq. guinea',
  'south sudan': 's. sudan', 'western sahara': 'w. sahara', 'bosnia and herzegovina': 'bosnia and herz.',
  'falkland islands': 'falkland is.', 'falkland islands (malvinas)': 'falkland is.', 'solomon islands': 'solomon is.',
  'french southern territories': 'fr. s. antarctic lands', 'north macedonia': 'macedonia', 'republic of north macedonia': 'macedonia',
  'swaziland': 'eswatini', 'czech republic': 'czechia', 'burma': 'myanmar', 'brunei darussalam': 'brunei',
  "lao people's democratic republic": 'laos', 'lao pdr': 'laos', 'northern cyprus': 'n. cyprus',
  'taiwan, province of china': 'taiwan', 'chinese taipei': 'taiwan', 'palestinian territory': 'palestine',
  'state of palestine': 'palestine', 'palestinian territory, occupied': 'palestine', 'united republic of tanzania': 'tanzania',
  'east timor': 'timor-leste', 'iran, islamic republic of': 'iran', 'syrian arab republic': 'syria',
  'venezuela, bolivarian republic of': 'venezuela', 'bolivia, plurinational state of': 'bolivia', 'republic of moldova': 'moldova',
  'viet nam': 'vietnam',
};

/** ISO 3166-1 alpha-2 to topology key (lowercase). world-110m country names. */
const ISO_TO_TOPOLOGY: Record<string, string> = {
  ad: 'andorra', ae: 'united arab emirates', af: 'afghanistan', al: 'albania', am: 'armenia', ao: 'angola',
  aq: 'antarctica', ar: 'argentina', at: 'austria', au: 'australia', az: 'azerbaijan', ba: 'bosnia and herz.',
  bd: 'bangladesh', be: 'belgium', bf: 'burkina faso', bg: 'bulgaria', bh: 'bhutan', bi: 'burundi', bj: 'benin',
  bn: 'brunei', bo: 'bolivia', br: 'brazil', bs: 'bahamas', bt: 'bhutan', bw: 'botswana', by: 'belarus',
  bz: 'belize', ca: 'canada', cd: 'dem. rep. congo', cf: 'central african rep.', cg: 'congo', ch: 'switzerland',
  ci: "côte d'ivoire", cl: 'chile', cm: 'cameroon', cn: 'china', co: 'colombia', cr: 'costa rica', cu: 'cuba',
  cz: 'czechia', de: 'germany', dj: 'djibouti', dk: 'denmark', do: 'dominican rep.', dz: 'algeria',
  ec: 'ecuador', ee: 'estonia', eg: 'egypt', eh: 'w. sahara', er: 'eritrea', es: 'spain', et: 'ethiopia',
  fi: 'finland', fj: 'fiji', fr: 'france', ga: 'gabon', gb: 'united kingdom', ge: 'georgia', gh: 'ghana',
  gm: 'gambia', gn: 'guinea', gq: 'eq. guinea', gr: 'greece', gt: 'guatemala', gw: 'guinea-bissau',
  gy: 'guyana', hn: 'honduras', hr: 'croatia', ht: 'haiti', hu: 'hungary', id: 'indonesia', ie: 'ireland',
  il: 'israel', in: 'india', iq: 'iraq', ir: 'iran', is: 'iceland', it: 'italy', jm: 'jamaica', jo: 'jordan',
  jp: 'japan', ke: 'kenya', kg: 'kyrgyzstan', kh: 'cambodia', kp: 'north korea', kr: 'south korea',
  kw: 'kuwait', kz: 'kazakhstan', la: 'laos', lb: 'lebanon', lk: 'sri lanka', lr: 'liberia', ls: 'lesotho',
  lt: 'lithuania', lu: 'luxembourg', lv: 'latvia', ly: 'libya', ma: 'morocco', md: 'moldova', me: 'montenegro',
  mg: 'madagascar', mk: 'macedonia', ml: 'mali', mm: 'myanmar', mn: 'mongolia', mr: 'mauritania', mt: 'malta',
  mw: 'malawi', mx: 'mexico', my: 'malaysia', mz: 'mozambique', na: 'namibia', ne: 'niger', ng: 'nigeria',
  ni: 'nicaragua', nl: 'netherlands', no: 'norway', np: 'nepal', nz: 'new zealand', om: 'oman', pa: 'panama',
  pe: 'peru', pg: 'papua new guinea', ph: 'philippines', pk: 'pakistan', pl: 'poland', pt: 'portugal',
  pr: 'puerto rico', ps: 'palestine', py: 'paraguay', qa: 'qatar', ro: 'romania', rs: 'serbia', ru: 'russia',
  rw: 'rwanda', sa: 'saudi arabia', sb: 'solomon is.', sd: 'sudan', se: 'sweden', si: 'slovenia', sk: 'slovakia',
  sl: 'sierra leone', sn: 'senegal', so: 'somalia', sr: 'suriname', ss: 's. sudan', sv: 'el salvador',
  sy: 'syria', sz: 'eswatini', td: 'chad', tg: 'togo', th: 'thailand', tj: 'tajikistan', tl: 'timor-leste',
  tm: 'turkmenistan', tn: 'tunisia', tr: 'turkey', tt: 'trinidad and tobago', tz: 'tanzania', ua: 'ukraine',
  ug: 'uganda', us: 'united states of america', uy: 'uruguay', uz: 'uzbekistan', ve: 'venezuela',
  vn: 'vietnam', vu: 'vanuatu', ye: 'yemen', za: 'south africa', zm: 'zambia', zw: 'zimbabwe',
};

function normalizeTopologyKey(name: string): string {
  const lower = name.toLowerCase().trim();
  return API_TO_TOPOLOGY[lower] ?? lower;
}

/**
 * Returns topology key (lowercase) for world-110m map matching.
 * Use when keying country data for the map: 2-letter ISO → topology key; longer names normalized via API_TO_TOPOLOGY.
 */
export function getCountryTopologyKey(isoOrName: string): string {
  if (!isoOrName || typeof isoOrName !== 'string') return '';
  const trimmed = isoOrName.trim();
  if (trimmed.length === 2) {
    return ISO_TO_TOPOLOGY[trimmed.toLowerCase()] ?? trimmed.toLowerCase();
  }
  return normalizeTopologyKey(trimmed);
}
