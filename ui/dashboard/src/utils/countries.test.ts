import { getCountryDisplayName, getCountryTopologyKey } from './countries';

describe('getCountryDisplayName', () => {
  it('maps a known lowercase ISO code to its display name', () => {
    expect(getCountryDisplayName('us')).toBe('United States');
  });

  it('is case-insensitive for ISO codes', () => {
    expect(getCountryDisplayName('US')).toBe('United States');
  });

  it('falls back to uppercased input for an unknown 2-letter code', () => {
    expect(getCountryDisplayName('zz')).toBe('ZZ');
  });

  it('returns a trimmed non-ISO value as-is', () => {
    expect(getCountryDisplayName('  United States  ')).toBe('United States');
  });

  it('returns an empty string for empty/undefined-ish input', () => {
    expect(getCountryDisplayName('')).toBe('');
  });
});

describe('getCountryTopologyKey', () => {
  it('maps a known ISO code to its topology key', () => {
    expect(getCountryTopologyKey('us')).toBe('united states of america');
  });

  it('is case-insensitive for ISO codes', () => {
    expect(getCountryTopologyKey('US')).toBe('united states of america');
  });

  it('normalizes a known alternate country name via API_TO_TOPOLOGY', () => {
    expect(getCountryTopologyKey('Russian Federation')).toBe('russia');
  });

  it('lowercases and trims an unrecognized name', () => {
    expect(getCountryTopologyKey('  Narnia  ')).toBe('narnia');
  });

  it('returns an empty string for empty input', () => {
    expect(getCountryTopologyKey('')).toBe('');
  });
});
