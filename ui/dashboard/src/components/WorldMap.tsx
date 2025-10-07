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

  // Color scale based on click data using neutral dashboard palette
  const getCountryColor = (countryName: string) => {
    const countryData = countryDataMap.get(countryName.toLowerCase());
    if (!countryData) {
      return '#f1f5f9'; // Light gray for countries with no data
    }

    const maxClicks = Math.max(...data.map(d => d.clicks));
    const intensity = countryData.clicks / maxClicks;
    
    // Neutral dashboard palette colors for countries with traffic
    if (intensity > 0.8) return '#64748b'; // Slate - High traffic
    if (intensity > 0.6) return '#6b7280'; // Gray - High-medium traffic
    if (intensity > 0.4) return '#71717a'; // Zinc - Medium traffic
    if (intensity > 0.2) return '#78716c'; // Stone - Low-medium traffic
    return '#7c2d12'; // Red-900 - Low traffic
  };

  const getCountryStroke = (countryName: string) => {
    const countryData = countryDataMap.get(countryName.toLowerCase());
    return countryData ? '#1f2937' : '#d1d5db';
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
                      },
                      hover: {
                        fill: getCountryColor(countryName),
                        stroke: '#1f2937',
                        strokeWidth: 1,
                        outline: 'none',
                      },
                      pressed: {
                        fill: getCountryColor(countryName),
                        stroke: '#1f2937',
                        strokeWidth: 1,
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
                   className="country-tooltip"
                   style={{
                     position: 'absolute',
                     left: mousePosition.x > 200 ? '10px' : 'auto',
                     right: mousePosition.x <= 200 ? '10px' : 'auto',
                     top: mousePosition.y > 200 ? '10px' : 'auto',
                     bottom: mousePosition.y <= 200 ? '10px' : 'auto',
                     zIndex: 1000
                   }}
                 >
                   <div className="tooltip-content">
                     <div className="country-name">{hoveredCountry}</div>
                     <div className="country-stats">
                       <div className="stat-item">
                         <span className="stat-label">Clicks:</span>
                         <span className="stat-value">
                           {countryDataMap.get(hoveredCountry.toLowerCase())?.clicks || 0}
                         </span>
                       </div>
                       <div className="stat-item">
                         <span className="stat-label">Percentage:</span>
                         <span className="stat-value">
                           {countryDataMap.get(hoveredCountry.toLowerCase())?.percentage || 0}%
                         </span>
                       </div>
                     </div>
                   </div>
                 </div>
               )}
    </div>
  );
};

export default WorldMap;
