import React from 'react';
import './App.css';
import Header from './components/Header';
import Hero from './components/Hero';
import Features from './components/Features';
import OpenSource from './components/OpenSource';
import HowItWorks from './components/HowItWorks';
import Stats from './components/Stats';
import Footer from './components/Footer';

function App() {
  return (
    <div className="App">
      <Header />
      <Hero />
      <Features />
      <OpenSource />
      <HowItWorks />
      <Stats />
      <Footer />
    </div>
  );
}

export default App;