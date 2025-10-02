import React from 'react';
import './Stats.css';

const Stats: React.FC = () => {
  const stats = [
    {
      number: '10M+',
      label: 'URLs Shortened',
      description: 'Links created by our users worldwide'
    },
    {
      number: '500K+',
      label: 'Active Users',
      description: 'Trusted by individuals and businesses'
    },
    {
      number: '99.9%',
      label: 'Uptime',
      description: 'Reliable service you can count on'
    },
    {
      number: '150+',
      label: 'Countries',
      description: 'Global reach across all continents'
    }
  ];

  return (
    <section className="stats section">
      <div className="container">
        <h2 className="section-title">Trusted Worldwide</h2>
        <p className="section-subtitle">
          Join millions of users who trust Shortas for their URL shortening needs
        </p>
        
        <div className="stats-grid">
          {stats.map((stat, index) => (
            <div key={index} className="stat-card">
              <div className="stat-number">{stat.number}</div>
              <div className="stat-label">{stat.label}</div>
              <div className="stat-description">{stat.description}</div>
            </div>
          ))}
        </div>
        
        <div className="cta-section">
          <h3 className="cta-title">Ready to get started?</h3>
          <p className="cta-subtitle">
            Join thousands of satisfied users and start shortening your URLs today
          </p>
          <a href="#signup" className="btn btn-primary btn-large">
            Start Free Trial
          </a>
        </div>
      </div>
    </section>
  );
};

export default Stats;
