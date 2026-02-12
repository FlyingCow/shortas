import React, { useState, useRef, useCallback, useMemo } from 'react';
import {
  ComposableMap,
  Geographies,
  Geography,
} from 'react-simple-maps';
import worldTopology from '../data/world-110m.json';
import { getCountryTopologyKey } from '../utils/countries';

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

/** Key for map lookup: topology name (lowercase). Uses shared country util. */
function dataKeyForTopology(item: CountryData): string {
  return getCountryTopologyKey(item.name);
}

const WorldMap: React.FC<WorldMapProps> = ({ data, height = 400 }) => {
  const [hoveredCountry, setHoveredCountry] = useState<string | null>(null);
  const [mousePosition, setMousePosition] = useState<{ x: number; y: number } | null>(null);
  const containerRef = useRef<HTMLDivElement>(null);

  // Key by topology name (lowercase) so geography lookups match. Resolves ISO codes from API.
  const countryDataMap = useMemo(
    () => new Map(data.map(item => [dataKeyForTopology(item), item])),
    [data]
  );

  const getCountryColor = useCallback((countryName: string) => {
    const match = countryDataMap.get(getCountryTopologyKey(countryName));
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
                {(countryDataMap.get(getCountryTopologyKey(hoveredCountry))?.clicks || 0).toLocaleString()}
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
                {countryDataMap.get(getCountryTopologyKey(hoveredCountry))?.percentage || 0}%
              </span>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default WorldMap;
