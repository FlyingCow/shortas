import React, { useState } from 'react';
import './Header.css';

const Header: React.FC = () => {
  const [isMenuOpen, setIsMenuOpen] = useState(false);

  return (
    <header className="header">
      <div className="container">
        <div className="nav-wrapper">
          <div className="logo">
            <h2>Shortas</h2>
          </div>
          
          <nav className={`nav ${isMenuOpen ? 'nav-open' : ''}`}>
            <a href="#features" className="nav-link">Features</a>
            <a href="#how-it-works" className="nav-link">How It Works</a>
            <a href="https://github.com/FlyingCow/shortas" target="_blank" rel="noopener noreferrer" className="nav-link">GitHub</a>
            <a href="#contact" className="nav-link">Contact</a>
          </nav>
          
          <div className="header-actions">
            <a href="#login" className="btn btn-secondary">Login</a>
            <a href="#signup" className="btn btn-primary">Get Started</a>
          </div>
          
          <button 
            className="mobile-menu-toggle"
            onClick={() => setIsMenuOpen(!isMenuOpen)}
          >
            <span></span>
            <span></span>
            <span></span>
          </button>
        </div>
      </div>
    </header>
  );
};

export default Header;
