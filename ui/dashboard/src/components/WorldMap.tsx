import React, { useState, useRef, useCallback, useMemo } from 'react';
import {
  ComposableMap,
  Geographies,
  Geography,
} from 'react-simple-maps';
import worldTopology from '../data/world-110m.json';

interface CountryData {
  name: string;
  clicks: number;
  percentage: number;
}

interface WorldMapProps {
  data: CountryData[];
  height?: number;
}

interface GeographyFeature {
  properties: {
    name?: string;
    [key: string]: any;
  };
  rsmKey: string;
}

// Maps common API / geo-IP country names to the topology file's names.
// Keys must be lowercase. Values must match topology names exactly (lowercased).
const API_TO_TOPO: Record<string, string> = {
  'united states': 'united states of america',
  'usa': 'united states of america',
  'us': 'united states of america',
  'russian federation': 'russia',
  'korea, republic of': 'south korea',
  "korea, democratic people's republic of": 'north korea',
  'democratic republic of the congo': 'dem. rep. congo',
  'democratic republic of congo': 'dem. rep. congo',
  'dr congo': 'dem. rep. congo',
  'congo, democratic republic of the': 'dem. rep. congo',
  'congo-kinshasa': 'dem. rep. congo',
  'republic of the congo': 'congo',
  'congo-brazzaville': 'congo',
  'ivory coast': "côte d'ivoire",
  "cote d'ivoire": "côte d'ivoire",
  'dominican republic': 'dominican rep.',
  'central african republic': 'central african rep.',
  'equatorial guinea': 'eq. guinea',
  'south sudan': 's. sudan',
  'western sahara': 'w. sahara',
  'bosnia and herzegovina': 'bosnia and herz.',
  'falkland islands': 'falkland is.',
  'falkland islands (malvinas)': 'falkland is.',
  'solomon islands': 'solomon is.',
  'french southern territories': 'fr. s. antarctic lands',
  'north macedonia': 'macedonia',
  'republic of north macedonia': 'macedonia',
  'swaziland': 'eswatini',
  'czech republic': 'czechia',
  'burma': 'myanmar',
  'brunei darussalam': 'brunei',
  "lao people's democratic republic": 'laos',
  'lao pdr': 'laos',
  'northern cyprus': 'n. cyprus',
  'taiwan, province of china': 'taiwan',
  'chinese taipei': 'taiwan',
  'palestinian territory': 'palestine',
  'state of palestine': 'palestine',
  'palestinian territory, occupied': 'palestine',
  'united republic of tanzania': 'tanzania',
  'east timor': 'timor-leste',
  'iran, islamic republic of': 'iran',
  'syrian arab republic': 'syria',
  'venezuela, bolivarian republic of': 'venezuela',
  'bolivia, plurinational state of': 'bolivia',
  'republic of moldova': 'moldova',
  'viet nam': 'vietnam',
};

function normalizeCountry(name: string): string {
  const lower = name.toLowerCase().trim();
  return API_TO_TOPO[lower] || lower;
}

const WorldMap: React.FC<WorldMapProps> = ({ data, height = 400 }) => {
  const [hoveredCountry, setHoveredCountry] = useState<string | null>(null);
  const [mousePosition, setMousePosition] = useState<{ x: number; y: number } | null>(null);
  const containerRef = useRef<HTMLDivElement>(null);

  // Normalize API country names to match topology file names
  const countryDataMap = useMemo(
    () => new Map(data.map(item => [normalizeCountry(item.name), item])),
    [data]
  );

  const getCountryColor = useCallback((countryName: string) => {
    const match = countryDataMap.get(normalizeCountry(countryName));
    if (!match) return '#e5e7eb';

    const maxClicks = Math.max(...data.map(d => d.clicks));
    const intensity = match.clicks / maxClicks;

    if (intensity > 0.8) return '#1e3a8a';
    if (intensity > 0.6) return '#2563eb';
    if (intensity > 0.4) return '#3b82f6';
    if (intensity > 0.2) return '#60a5fa';
    return '#93c5fd';
  }, [countryDataMap, data]);

  return (
    <div
      ref={containerRef}
      className="world-map-container"
      style={{ height: `${height}px`, width: '100%', minHeight: 'auto' }}
    >
      <ComposableMap
        projection="geoEqualEarth"
        projectionConfig={{ scale: 160 }}
        width={800}
        height={400}
        style={{ width: '100%', height: '100%' }}
      >
        <Geographies geography={worldTopology as any}>
          {({ geographies }: { geographies: GeographyFeature[] }) =>
            geographies.map((geo: GeographyFeature) => {
              const countryName = geo.properties.name || '';
              return (
                <Geography
                  key={geo.rsmKey}
                  geography={geo}
                  onMouseEnter={(event) => {
                    setHoveredCountry(countryName);
                    const rect = containerRef.current?.getBoundingClientRect();
                    if (rect) {
                      setMousePosition({
                        x: event.clientX - rect.left,
                        y: event.clientY - rect.top,
                      });
                    }
                  }}
                  onMouseLeave={() => {
                    setHoveredCountry(null);
                    setMousePosition(null);
                  }}
                  style={{
                    default: {
                      fill: getCountryColor(countryName),
                      stroke: '#d1d5db',
                      strokeWidth: 0.5,
                      outline: 'none',
                    },
                    hover: {
                      fill: getCountryColor(countryName),
                      stroke: '#1e40af',
                      strokeWidth: 1.5,
                      outline: 'none',
                      filter: 'brightness(1.1)',
                    },
                    pressed: {
                      fill: getCountryColor(countryName),
                      stroke: '#1e40af',
                      strokeWidth: 1.5,
                      outline: 'none',
                    },
                  }}
                />
              );
            })
          }
        </Geographies>
      </ComposableMap>

      {hoveredCountry && mousePosition && (
        <div
          style={{
            position: 'absolute',
            left: mousePosition.x > (containerRef.current?.clientWidth ?? 0) / 2 ? '10px' : 'auto',
            right: mousePosition.x <= (containerRef.current?.clientWidth ?? 0) / 2 ? '10px' : 'auto',
            top: mousePosition.y > height / 2 ? '10px' : 'auto',
            bottom: mousePosition.y <= height / 2 ? '10px' : 'auto',
            zIndex: 1000,
            backgroundColor: 'var(--bg-primary)',
            border: '1px solid var(--border-primary)',
            borderRadius: '6px',
            padding: '0.75rem',
            boxShadow: '0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06)',
            pointerEvents: 'none',
            minWidth: '180px'
          }}
        >
          <div style={{
            fontSize: '0.9rem',
            fontWeight: '600',
            marginBottom: '0.5rem',
            color: 'var(--text-primary)',
            borderBottom: '1px solid var(--border-secondary)',
            paddingBottom: '0.375rem'
          }}>
            {hoveredCountry}
          </div>
          <div style={{ fontSize: '0.8rem' }}>
            <div style={{
              display: 'flex',
              justifyContent: 'space-between',
              marginBottom: '0.25rem'
            }}>
              <span style={{ color: 'var(--text-muted)' }}>Clicks:</span>
              <span style={{
                fontWeight: '600',
                color: 'var(--text-primary)'
              }}>
                {(countryDataMap.get(normalizeCountry(hoveredCountry))?.clicks || 0).toLocaleString()}
              </span>
            </div>
            <div style={{
              display: 'flex',
              justifyContent: 'space-between'
            }}>
              <span style={{ color: 'var(--text-muted)' }}>Percentage:</span>
              <span style={{
                fontWeight: '600',
                color: 'var(--primary-500)'
              }}>
                {countryDataMap.get(normalizeCountry(hoveredCountry))?.percentage || 0}%
              </span>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default WorldMap;
