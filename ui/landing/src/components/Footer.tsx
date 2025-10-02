import React from 'react';
import './Footer.css';

const Footer: React.FC = () => {
  return (
    <footer className="footer">
      <div className="container">
        <div className="footer-content">
          <div className="footer-section">
            <div className="footer-logo">
              <h3>Shortas</h3>
              <p>The fastest and most reliable open-source URL shortening service on the web. Built with ❤️ by the community.</p>
            </div>
            <div className="social-links">
              <a href="#" aria-label="Twitter">🐦</a>
              <a href="#" aria-label="LinkedIn">💼</a>
              <a href="https://github.com/FlyingCow/shortas" target="_blank" rel="noopener noreferrer" aria-label="GitHub">🐙</a>
              <a href="#" aria-label="Discord">💬</a>
            </div>
          </div>
          
          <div className="footer-section">
            <h4>Product</h4>
            <ul>
              <li><a href="#features">Features</a></li>
              <li><a href="#pricing">Pricing</a></li>
              <li><a href="https://github.com/FlyingCow/shortas/blob/main/README.md" target="_blank" rel="noopener noreferrer">API Documentation</a></li>
              <li><a href="https://github.com/FlyingCow/shortas" target="_blank" rel="noopener noreferrer">Source Code</a></li>
            </ul>
          </div>
          
          <div className="footer-section">
            <h4>Company</h4>
            <ul>
              <li><a href="#about">About Us</a></li>
              <li><a href="#blog">Blog</a></li>
              <li><a href="#careers">Careers</a></li>
              <li><a href="#contact">Contact</a></li>
            </ul>
          </div>
          
          <div className="footer-section">
            <h4>Community</h4>
            <ul>
              <li><a href="https://github.com/FlyingCow/shortas/issues" target="_blank" rel="noopener noreferrer">Report Issues</a></li>
              <li><a href="https://github.com/FlyingCow/shortas/discussions" target="_blank" rel="noopener noreferrer">Discussions</a></li>
              <li><a href="https://github.com/FlyingCow/shortas/blob/main/CONTRIBUTING.md" target="_blank" rel="noopener noreferrer">Contributing</a></li>
              <li><a href="https://github.com/FlyingCow/shortas/releases" target="_blank" rel="noopener noreferrer">Releases</a></li>
            </ul>
          </div>
        </div>
        
        <div className="footer-bottom">
          <div className="footer-copyright">
            <p>&copy; 2025 Shortas. All rights reserved.</p>
          </div>
          <div className="footer-links">
            <a href="#privacy">Privacy</a>
            <a href="#terms">Terms</a>
            <a href="#cookies">Cookies</a>
          </div>
        </div>
      </div>
    </footer>
  );
};

export default Footer;
