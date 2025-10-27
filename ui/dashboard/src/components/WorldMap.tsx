import React, { useState } from 'react';
import {
  ComposableMap,
  Geographies,
  Geography,
  ZoomableGroup
} from 'react-simple-maps';

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
    NAME?: string;
    NAME_EN?: string;
    name?: string;
    NAME_LONG?: string;
    ADMIN?: string;
    ISO_A3?: string;
    [key: string]: any;
  };
  rsmKey: string;
}

const WorldMap: React.FC<WorldMapProps> = ({ data, height = 400 }) => {
  const [hoveredCountry, setHoveredCountry] = useState<string | null>(null);
  const [mousePosition, setMousePosition] = useState<{ x: number; y: number } | null>(null);
  
  // Create a map of country names to click data
  const countryDataMap = new Map(
    data.map(item => [item.name.toLowerCase(), item])
  );

  console.log('WorldMap data:', data);
  console.log('Country data map:', countryDataMap);

  // Color scale based on click data using dashboard primary colors
  const getCountryColor = (countryName: string) => {
    const countryData = countryDataMap.get(countryName.toLowerCase());
    if (!countryData) {
      return 'var(--bg-secondary)'; // Light gray for countries with no data
    }

    const maxClicks = Math.max(...data.map(d => d.clicks));
    const intensity = countryData.clicks / maxClicks;

    // Primary color gradient for countries with traffic
    if (intensity > 0.8) return '#1e3a8a'; // Dark blue - Highest traffic
    if (intensity > 0.6) return '#2563eb'; // Blue - High traffic
    if (intensity > 0.4) return '#3b82f6'; // Medium blue - Medium traffic
    if (intensity > 0.2) return '#60a5fa'; // Light blue - Low-medium traffic
    return '#93c5fd'; // Very light blue - Low traffic
  };

  const getCountryStroke = (countryName: string) => {
    const countryData = countryDataMap.get(countryName.toLowerCase());
    return countryData ? 'var(--border-primary)' : 'var(--border-secondary)';
  };

  return (
    <div className="world-map-container" style={{ height: `${height}px`, width: '100%' }}>
      <ComposableMap
        projection="geoEquirectangular"
        projectionConfig={{
          scale: 200,
          center: [0, 0]
        }}
        style={{ width: '100%', height: '100%' }}
      >
        <ZoomableGroup>
          <Geographies geography="/world-110m.json">
            {({ geographies }: { geographies: GeographyFeature[] }) => {
              console.log('Geographies loaded:', geographies.length);
              return geographies.map((geo: GeographyFeature) => {
                const countryName = geo.properties.NAME || geo.properties.NAME_EN || geo.properties.name || geo.properties.NAME_LONG || geo.properties.ADMIN || '';
                const countryData = countryDataMap.get(countryName.toLowerCase());
                console.log('Rendering country:', countryName);
                return (
                  <Geography
                    key={geo.rsmKey}
                    geography={geo}
                    fill={getCountryColor(countryName)}
                    stroke={getCountryStroke(countryName)}
                    strokeWidth={0.5}
                             onMouseEnter={(event) => {
                               setHoveredCountry(countryName);
                               const rect = event.currentTarget.getBoundingClientRect();
                               const containerRect = event.currentTarget.closest('.world-map-container')?.getBoundingClientRect();
                               if (containerRect) {
                                 const x = event.clientX - containerRect.left;
                                 const y = event.clientY - containerRect.top;
                                 setMousePosition({ x, y });
                               }
                             }}
                             onMouseLeave={() => {
                               setHoveredCountry(null);
                               setMousePosition(null);
                             }}
                    style={{
                      default: {
                        fill: getCountryColor(countryName),
                        stroke: getCountryStroke(countryName),
                        strokeWidth: 0.5,
                        outline: 'none',
                        transition: 'all 0.2s ease',
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
              });
            }}
          </Geographies>
        </ZoomableGroup>
      </ComposableMap>
      
               {/* Country Tooltip */}
               {hoveredCountry && mousePosition && (
                 <div
                   style={{
                     position: 'absolute',
                     left: mousePosition.x > 200 ? '10px' : 'auto',
                     right: mousePosition.x <= 200 ? '10px' : 'auto',
                     top: mousePosition.y > 200 ? '10px' : 'auto',
                     bottom: mousePosition.y <= 200 ? '10px' : 'auto',
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
                         {(countryDataMap.get(hoveredCountry.toLowerCase())?.clicks || 0).toLocaleString()}
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
                         {countryDataMap.get(hoveredCountry.toLowerCase())?.percentage || 0}%
                       </span>
                     </div>
                   </div>
                 </div>
               )}
    </div>
  );
};

export default WorldMap;
