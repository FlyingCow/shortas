import React, { useState } from 'react';
import './Header.css';
import { GithubIcon, MenuIcon, CloseIcon } from './Icons';

const Header: React.FC = () => {
  const [isMenuOpen, setIsMenuOpen] = useState(false);

  return (
    <header className="header">
      <div className="container">
        <div className="nav-wrapper">
          <a href="#top" className="logo">
            Shortas
          </a>

          <nav className={`nav ${isMenuOpen ? 'nav-open' : ''}`}>
            <a href="#features" className="nav-link" onClick={() => setIsMenuOpen(false)}>Features</a>
            <a href="#how-it-works" className="nav-link" onClick={() => setIsMenuOpen(false)}>How It Works</a>
            <a href="#open-source" className="nav-link" onClick={() => setIsMenuOpen(false)}>Open Source</a>
            <a href="https://shortas.work/" target="_blank" rel="noopener noreferrer" className="nav-link">Docs</a>
            <a
              href="https://github.com/FlyingCow/shortas"
              target="_blank"
              rel="noopener noreferrer"
              className="nav-link nav-link-mobile-only"
            >
              GitHub
            </a>
          </nav>

          <div className="header-actions">
            <a
              href="https://github.com/FlyingCow/shortas"
              target="_blank"
              rel="noopener noreferrer"
              className="icon-link"
              aria-label="View source on GitHub"
            >
              <GithubIcon width={20} height={20} />
            </a>
            <a href="https://shortas.work/getting-started/" target="_blank" rel="noopener noreferrer" className="btn btn-primary">
              Get Started
            </a>
          </div>

          <button
            className="mobile-menu-toggle"
            onClick={() => setIsMenuOpen(!isMenuOpen)}
            aria-label="Toggle menu"
            aria-expanded={isMenuOpen}
          >
            {isMenuOpen ? <CloseIcon width={22} height={22} /> : <MenuIcon width={22} height={22} />}
          </button>
        </div>
      </div>
    </header>
  );
};

export default Header;
