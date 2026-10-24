import React from 'react';
import './Hero.css';
import { GithubIcon, ArrowRightIcon, ZapIcon, CodeIcon, ServerIcon } from './Icons';

const Hero: React.FC = () => {
  return (
    <section className="hero" id="top">
      <div className="hero-glow" aria-hidden="true" />
      <div className="container">
        <div className="hero-content">
          <div className="hero-text">
            <div className="hero-badge">
              <ZapIcon width={14} height={14} />
              Rust-powered redirect engine
            </div>
            <h1 className="hero-title">
              URL shortening,
              <span className="gradient-text"> built for speed</span>
            </h1>
            <p className="hero-subtitle">
              Shortas is an open-source, self-hostable link shortener with conditional
              routing, real-time click analytics, QR codes, and automatic TLS for your
              own custom domains.
            </p>
            <div className="hero-actions">
              <a href="https://shortas.work/getting-started/" target="_blank" rel="noopener noreferrer" className="btn btn-primary btn-large">
                Get Started
                <ArrowRightIcon width={18} height={18} />
              </a>
              <a href="https://github.com/FlyingCow/shortas" target="_blank" rel="noopener noreferrer" className="btn btn-secondary btn-large">
                <GithubIcon width={18} height={18} />
                View on GitHub
              </a>
            </div>
            <div className="hero-highlights">
              <div className="highlight">
                <CodeIcon width={18} height={18} />
                <span>MIT licensed</span>
              </div>
              <div className="highlight">
                <ServerIcon width={18} height={18} />
                <span>Self-hostable anywhere</span>
              </div>
              <div className="highlight">
                <ZapIcon width={18} height={18} />
                <span>Rust &amp; Tokio core</span>
              </div>
            </div>
          </div>
          <div className="hero-visual">
            <div className="url-demo">
              <div className="url-demo-header">
                <span className="dot dot-red" />
                <span className="dot dot-yellow" />
                <span className="dot dot-green" />
              </div>
              <div className="url-input">
                <span className="url-label">Long URL</span>
                <div className="url-box">
                  https://example.com/campaign/summer?utm_source=newsletter&amp;utm_id=204
                </div>
              </div>
              <div className="arrow-row">
                <div className="arrow">↓</div>
                <span className="arrow-label">conditional routing · click tracking</span>
              </div>
              <div className="url-output">
                <span className="url-label">Short URL</span>
                <div className="url-box short">
                  shortas.work/s-2a9f
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
