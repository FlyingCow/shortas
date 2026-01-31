import React from 'react';
import { Github } from 'lucide-react';
import './Footer.css';

const Footer: React.FC = () => {
  const currentYear = new Date().getFullYear();

  return (
    <footer className="app-footer">
      <span>&copy; {currentYear} Shortas</span>
      <div className="footer-links">
        <a href="#terms">Terms</a>
        <a href="#privacy">Privacy</a>
        <a href="https://github.com" target="_blank" rel="noopener noreferrer" aria-label="GitHub">
          <Github size={14} />
        </a>
      </div>
    </footer>
  );
};

export default Footer;
