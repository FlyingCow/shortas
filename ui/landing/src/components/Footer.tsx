import React from 'react';
import './Footer.css';
import { GithubIcon } from './Icons';

const Footer: React.FC = () => {
  const year = new Date().getFullYear();

  return (
    <footer className="footer">
      <div className="container">
        <div className="footer-content">
          <div className="footer-section footer-brand">
            <div className="footer-logo">
              <h3>Shortas</h3>
              <p>An open-source, self-hostable URL shortener built in Rust.</p>
            </div>
            <div className="social-links">
              <a href="https://github.com/FlyingCow/shortas" target="_blank" rel="noopener noreferrer" aria-label="GitHub">
                <GithubIcon width={18} height={18} />
              </a>
            </div>
          </div>

          <div className="footer-section">
            <h4>Documentation</h4>
            <ul>
              <li><a href="https://shortas.work/getting-started/" target="_blank" rel="noopener noreferrer">Getting Started</a></li>
              <li><a href="https://shortas.work/architecture/" target="_blank" rel="noopener noreferrer">Architecture</a></li>
              <li><a href="https://shortas.work/api/" target="_blank" rel="noopener noreferrer">API Reference</a></li>
              <li><a href="https://shortas.work/deployment/" target="_blank" rel="noopener noreferrer">Deployment</a></li>
            </ul>
          </div>

          <div className="footer-section">
            <h4>Project</h4>
            <ul>
              <li><a href="https://github.com/FlyingCow/shortas" target="_blank" rel="noopener noreferrer">Source Code</a></li>
              <li><a href="https://github.com/FlyingCow/shortas/issues" target="_blank" rel="noopener noreferrer">Report an Issue</a></li>
              <li><a href="https://github.com/FlyingCow/shortas/discussions" target="_blank" rel="noopener noreferrer">Discussions</a></li>
              <li><a href="https://shortas.work/development/" target="_blank" rel="noopener noreferrer">Contributing</a></li>
            </ul>
          </div>

          <div className="footer-section">
            <h4>About</h4>
            <ul>
              <li><a href="https://shortas.work/about/" target="_blank" rel="noopener noreferrer">About Shortas</a></li>
              <li><a href="https://github.com/FlyingCow/shortas/blob/main/LICENSE" target="_blank" rel="noopener noreferrer">MIT License</a></li>
              <li><a href="https://github.com/FlyingCow/shortas/releases" target="_blank" rel="noopener noreferrer">Releases</a></li>
            </ul>
          </div>
        </div>

        <div className="footer-bottom">
          <p>&copy; {year} Shortas. Released under the MIT License.</p>
        </div>
      </div>
    </footer>
  );
};

export default Footer;
