import React from 'react';
import { AlertTriangle, Database } from 'lucide-react';
import { useMockData, isDevelopment } from '../config/development';
import './DevelopmentBanner.css';

const DevelopmentBanner: React.FC = () => {
  if (!isDevelopment) return null;

  return (
    <div className="development-banner">
      <div className="banner-content">
        <AlertTriangle className="banner-icon" />
        <div className="banner-text">
          <strong>Development Mode</strong>
          {useMockData && (
            <>
              {' '}- Using mock data
              <Database className="mock-icon" />
            </>
          )}
        </div>
        {useMockData && (
          <div className="banner-hint">
            Set REACT_APP_USE_MOCK_DATA=false to use real API
          </div>
        )}
      </div>
    </div>
  );
};

export default DevelopmentBanner;


