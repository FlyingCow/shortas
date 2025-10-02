import React from 'react';
import './OpenSource.css';

const OpenSource: React.FC = () => {
  return (
    <section className="open-source section">
      <div className="container">
        <div className="open-source-content">
          <div className="open-source-text">
            <h2 className="section-title">Built by the Community</h2>
            <p className="section-subtitle">
              Shortas is completely open source, built with transparency and community collaboration at its core.
            </p>
            
            <div className="open-source-features">
              <div className="os-feature">
                <div className="os-icon">🔓</div>
                <div className="os-content">
                  <h3>Fully Open Source</h3>
                  <p>All code is available on GitHub under MIT license. No hidden algorithms or black boxes.</p>
                </div>
              </div>
              
              <div className="os-feature">
                <div className="os-icon">🤝</div>
                <div className="os-content">
                  <h3>Community Driven</h3>
                  <p>Built by developers, for developers. Contributions welcome from anyone who wants to improve the platform.</p>
                </div>
              </div>
              
              <div className="os-feature">
                <div className="os-icon">🏠</div>
                <div className="os-content">
                  <h3>Self-Hostable</h3>
                  <p>Deploy your own instance with full control over your data and customization options.</p>
                </div>
              </div>
            </div>
            
            <div className="github-stats">
              <div className="github-stat">
                <span className="stat-number">⭐ 1.2k</span>
                <span className="stat-label">GitHub Stars</span>
              </div>
              <div className="github-stat">
                <span className="stat-number">🍴 200+</span>
                <span className="stat-label">Forks</span>
              </div>
              <div className="github-stat">
                <span className="stat-number">👥 50+</span>
                <span className="stat-label">Contributors</span>
              </div>
            </div>
            
            <div className="github-actions">
              <a 
                href="https://github.com/FlyingCow/shortas" 
                target="_blank" 
                rel="noopener noreferrer" 
                className="btn btn-primary"
              >
                View Source Code
              </a>
              <a 
                href="https://github.com/FlyingCow/shortas/blob/main/CONTRIBUTING.md" 
                target="_blank" 
                rel="noopener noreferrer" 
                className="btn btn-secondary"
              >
                Start Contributing
              </a>
            </div>
          </div>
          
          <div className="github-preview">
            <div className="github-card">
              <div className="github-header">
                <div className="github-avatar">🐙</div>
                <div className="github-info">
                  <h4>FlyingCow/shortas</h4>
                  <p>Lightning-fast URL shortener built with Rust</p>
                </div>
              </div>
              
              <div className="github-stats-inline">
                <span className="github-lang">🦀 Rust</span>
                <span className="github-stars">⭐ 1,234</span>
                <span className="github-forks">🍴 200</span>
              </div>
              
              <div className="github-topics">
                <span className="topic">url-shortener</span>
                <span className="topic">rust</span>
                <span className="topic">web-service</span>
                <span className="topic">open-source</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>
  );
};

export default OpenSource;
