import React from 'react';
import './Features.css';
import {
  ZapIcon,
  BarChartIcon,
  LinkIcon,
  ShieldIcon,
  GlobeIcon,
  CodeIcon,
} from './Icons';

const Features: React.FC = () => {
  const features = [
    {
      icon: <ZapIcon />,
      title: 'Built for Speed',
      description: 'A Rust and Tokio redirect engine with in-memory caching keeps the hot path fast, even under heavy traffic.'
    },
    {
      icon: <BarChartIcon />,
      title: 'Real-Time Analytics',
      description: 'Every click is enriched with geography, device, and browser data and streamed into ClickHouse for instant dashboards.'
    },
    {
      icon: <GlobeIcon />,
      title: 'Conditional Routing',
      description: 'Send visitors to different destinations based on country, device, browser, or OS - all from one short link.'
    },
    {
      icon: <LinkIcon />,
      title: 'Custom Domains',
      description: 'Bring your own domain with automatic TLS certificates issued and renewed via Let’s Encrypt.'
    },
    {
      icon: <ShieldIcon />,
      title: 'Safe by Default',
      description: 'Destinations are checked against Google Safe Browsing on a schedule, with unsafe links blocked automatically.'
    },
    {
      icon: <CodeIcon />,
      title: 'Open Source',
      description: 'MIT-licensed with a documented REST API. Self-host it, read the code, or contribute a feature back.'
    }
  ];

  return (
    <section id="features" className="features section">
      <div className="container">
        <span className="section-eyebrow">Features</span>
        <h2 className="section-title">Everything a short link needs</h2>
        <p className="section-subtitle">
          From the redirect path to the analytics pipeline, built as a set of focused Rust services.
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
