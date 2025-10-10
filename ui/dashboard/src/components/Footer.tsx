import React from 'react';
import { 
  Github, 
  Twitter, 
  Linkedin, 
  Mail, 
  ExternalLink,
  Heart,
  Code,
  Shield,
  Zap
} from 'lucide-react';
import Logo from './Logo';
import './Footer.css';

const Footer: React.FC = () => {
  const currentYear = new Date().getFullYear();

  return (
    <footer className="dashboard-footer">
      <div className="footer-content">
        <div className="footer-main">
          <div className="footer-brand">
            <div className="footer-logo">
              <Logo size={24} />
              <span>Shortas</span>
            </div>
            <p className="footer-description">
              High-performance URL shortening and analytics platform built for scale.
            </p>
            <div className="footer-social">
              <a href="https://github.com" target="_blank" rel="noopener noreferrer" className="social-link">
                <Github size={18} />
              </a>
              <a href="https://twitter.com" target="_blank" rel="noopener noreferrer" className="social-link">
                <Twitter size={18} />
              </a>
              <a href="https://linkedin.com" target="_blank" rel="noopener noreferrer" className="social-link">
                <Linkedin size={18} />
              </a>
              <a href="mailto:support@shortas.com" className="social-link">
                <Mail size={18} />
              </a>
            </div>
          </div>

          <div className="footer-links">
            <div className="footer-section">
              <h4>Product</h4>
              <ul>
                <li><a href="#features">Features</a></li>
                <li><a href="#pricing">Pricing</a></li>
                <li><a href="#api">API</a></li>
                <li><a href="#integrations">Integrations</a></li>
              </ul>
            </div>

            <div className="footer-section">
              <h4>Resources</h4>
              <ul>
                <li><a href="#documentation">Documentation</a></li>
                <li><a href="#guides">Guides</a></li>
                <li><a href="#examples">Examples</a></li>
                <li><a href="#tutorials">Tutorials</a></li>
              </ul>
            </div>

            <div className="footer-section">
              <h4>Support</h4>
              <ul>
                <li><a href="#help">Help Center</a></li>
                <li><a href="#contact">Contact Us</a></li>
                <li><a href="#status">Status</a></li>
                <li><a href="#community">Community</a></li>
              </ul>
            </div>

            <div className="footer-section">
              <h4>Company</h4>
              <ul>
                <li><a href="#about">About</a></li>
                <li><a href="#blog">Blog</a></li>
                <li><a href="#careers">Careers</a></li>
                <li><a href="#privacy">Privacy</a></li>
              </ul>
            </div>
          </div>
        </div>

        <div className="footer-bottom">
          <div className="footer-bottom-left">
            <p>&copy; {currentYear} Shortas. All rights reserved.</p>
            <div className="footer-bottom-links">
              <a href="#terms">Terms of Service</a>
              <a href="#privacy">Privacy Policy</a>
              <a href="#cookies">Cookie Policy</a>
            </div>
          </div>
          <div className="footer-bottom-right">
            <div className="footer-tech">
              <span className="tech-label">Built with</span>
              <div className="tech-stack">
                <span className="tech-item">
                  <Code size={14} />
                  Rust
                </span>
                <span className="tech-item">
                  <Shield size={14} />
                  Security
                </span>
                <span className="tech-item">
                  <Zap size={14} />
                  Performance
                </span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </footer>
  );
};

export default Footer;
