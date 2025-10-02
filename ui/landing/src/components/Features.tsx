import React from 'react';
import './Features.css';

const Features: React.FC = () => {
  const features = [
    {
      icon: '⚡',
      title: 'Lightning Fast',
      description: 'Generate short URLs in milliseconds with our optimized infrastructure and global CDN.'
    },
    {
      icon: '📊',
      title: 'Advanced Analytics',
      description: 'Track clicks, geographic data, referrers, and device information with detailed insights.'
    },
    {
      icon: '🎯',
      title: 'Custom Domains',
      description: 'Use your own branded domain for professional-looking short links that build trust.'
    },
    {
      icon: '🔒',
      title: 'Enterprise Security',
      description: 'Bank-level security with SSL encryption, password protection, and expiration dates.'
    },
    {
      icon: '🌍',
      title: 'Global Reach',
      description: 'Worldwide infrastructure ensures fast redirects from anywhere on the planet.'
    },
    {
      icon: '🔧',
      title: 'Open Source',
      description: 'Fully open-source project with comprehensive REST API. Contribute, customize, or self-host your own instance.'
    }
  ];

  return (
    <section id="features" className="features section">
      <div className="container">
        <h2 className="section-title">Powerful Features</h2>
        <p className="section-subtitle">
          Everything you need to create, manage, and track your short links effectively
        </p>
        
        <div className="features-grid">
          {features.map((feature, index) => (
            <div key={index} className="feature-card">
              <div className="feature-icon">{feature.icon}</div>
              <h3 className="feature-title">{feature.title}</h3>
              <p className="feature-description">{feature.description}</p>
            </div>
          ))}
        </div>
      </div>
    </section>
  );
};

export default Features;
