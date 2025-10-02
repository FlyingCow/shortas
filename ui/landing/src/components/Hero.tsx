import React from 'react';
import './Hero.css';

const Hero: React.FC = () => {
  return (
    <section className="hero">
      <div className="container">
        <div className="hero-content">
          <div className="hero-text">
            <h1 className="hero-title">
              Shorten URLs with
              <span className="gradient-text"> Lightning Speed</span>
            </h1>
            <p className="hero-subtitle">
              Create short, memorable links that drive engagement. Track clicks, 
              analyze performance, and optimize your marketing campaigns with our 
              powerful open-source URL shortening platform.
            </p>
            <div className="hero-actions">
              <a href="#signup" className="btn btn-primary btn-large">
                Start Shortening Free
              </a>
              <a href="https://github.com/FlyingCow/shortas" target="_blank" rel="noopener noreferrer" className="btn btn-secondary btn-large">
                View on GitHub
              </a>
            </div>
            <div className="hero-stats">
              <div className="stat">
                <span className="stat-number">10M+</span>
                <span className="stat-label">Links Created</span>
              </div>
              <div className="stat">
                <span className="stat-number">500K+</span>
                <span className="stat-label">Active Users</span>
              </div>
              <div className="stat">
                <span className="stat-number">99.9%</span>
                <span className="stat-label">Uptime</span>
              </div>
            </div>
          </div>
          <div className="hero-visual">
            <div className="url-demo">
              <div className="url-input">
                <span className="url-label">Long URL:</span>
                <div className="url-box">
                  https://example.com/very/long/url/with/many/parameters?utm_source=...
                </div>
              </div>
              <div className="arrow">↓</div>
              <div className="url-output">
                <span className="url-label">Short URL:</span>
                <div className="url-box short">
                  shortas.com/abc123
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>
  );
};

export default Hero;
